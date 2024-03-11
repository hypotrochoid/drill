use {
    anyhow::Result,
    cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE},
    jieba_rs::Jieba,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    tantivy::{
        collector::TopDocs, doc, query::QueryParser, schema::*, Index, IndexReader, IndexWriter,
        ReloadPolicy,
    },
};

#[derive(Serialize, Deserialize)]
pub struct GenericDictEntry {
    // dict
    simplified: Option<String>,     // s
    traditional: Option<String>,    //t
    zh_definition: Option<String>,  //d
    eng_definition: Option<String>, //d
    english: Option<String>,        //e
    synonym: Vec<String>,           //syn
    anyonym: Vec<String>,           //ant
    ipa: Option<String>,            //ipa
    // wiki
    url: Option<String>,   //url
    title: Option<String>, //title
    text: Option<String>,  //text
    // tatoeba
    english_sentence: Option<String>, //english
    chinese_sentence: Option<String>, //chinese

    // news
    keywords: Option<String>,    //keywords
    description: Option<String>, //desc
    // : Option<String>, //title
    source: Option<String>,  //source
    time: Option<String>,    //time
    content: Option<String>, //content
    // baike
    category: Option<String>, //category
    // description // desc
    answer: Option<String>, //answer
}

pub struct DictIndex {
    index: Index,
    reader: IndexReader,
    schema: Schema,
}

impl DictIndex {
    fn index(&mut self, documents: &[serde_json::Map<String, serde_json::Value>]) -> Result<()>{
        let mut index_writer: IndexWriter = self.index.writer(50_000_000)?;

        for doc in documents {
            let t_doc = self.schema.json_object_to_doc(doc.clone())?;
            index_writer.add_document(t_doc);
        }

        index_writer.commit()?;

        Ok(())
    }

    fn search(&self, query_str: &str, top_k: usize, fields: &[&str]) -> Result<Vec<Document>>{
        let searcher = self.reader.searcher();
        let q_fields: Vec<_> = fields
            .iter()
            .map(|f| self.schema.get_field(f).unwrap())
            .collect();

        let query_parser = QueryParser::for_index(&self.index, q_fields);
        let query = query_parser.parse_query(query_str)?;

        let results = searcher.search(&query, &TopDocs::with_limit(top_k))?;
        let mut retv = Vec::with_capacity(top_k);

        for (score, doc_address) in results {
            // Note that the score is not lower for the fuzzy hit.
            // There's an issue open for that: https://github.com/quickwit-oss/tantivy/issues/563
            let retrieved_doc: Document = searcher.doc(doc_address)?;
            retv.push(retrieved_doc)
        }

        Ok(retv)
    }

    fn open(path: &str) -> Result<Self> {
        let index = Index::open_in_dir(path)?;
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        let schema = index.schema();

        Ok(DictIndex {
            index,
            schema,
            reader,
        })
    }

    fn new(index_path: &str) -> Result<Self> {
        let mut schema_builder = SchemaBuilder::default();

        let english_indexing =
            TextFieldIndexing::default().set_index_option(IndexRecordOption::WithFreqsAndPositions);

        let chinese_indexing = TextFieldIndexing::default()
            .set_tokenizer(CANG_JIE) // Set custom tokenizer
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);

        let eng_text_options = TextOptions::default()
            .set_indexing_options(english_indexing)
            .set_stored();

        let zh_text_options = TextOptions::default()
            .set_indexing_options(chinese_indexing)
            .set_stored();

        let _ = schema_builder.add_text_field("simplified", zh_text_options.clone());
        let _ = schema_builder.add_text_field("traditional", zh_text_options.clone());
        let _ = schema_builder.add_text_field("zh_definition", zh_text_options.clone());
        let _ = schema_builder.add_text_field("eng_definition", eng_text_options.clone());
        let _ = schema_builder.add_text_field("english", eng_text_options.clone());
        let _ = schema_builder.add_text_field("synonym", zh_text_options.clone());
        let _ = schema_builder.add_text_field("antonym", zh_text_options.clone());
        let _ = schema_builder.add_text_field("ipa", eng_text_options.clone());
        let _ = schema_builder.add_text_field("url", eng_text_options.clone());
        let _ = schema_builder.add_text_field("title", zh_text_options.clone());
        let _ = schema_builder.add_text_field("text", zh_text_options.clone());
        let _ = schema_builder.add_text_field("english_sentence", eng_text_options.clone());
        let _ = schema_builder.add_text_field("chinese_sentence", zh_text_options.clone());
        let _ = schema_builder.add_text_field("keywords", zh_text_options.clone());
        let _ = schema_builder.add_text_field("description", zh_text_options.clone());
        let _ = schema_builder.add_text_field("source", zh_text_options.clone());
        let _ = schema_builder.add_text_field("time", eng_text_options.clone());
        let _ = schema_builder.add_text_field("content", zh_text_options.clone());
        let _ = schema_builder.add_text_field("category", zh_text_options.clone());
        let _ = schema_builder.add_text_field("answer", zh_text_options.clone());

        let schema = schema_builder.build();

        let index = Index::create_in_dir(index_path, schema.clone())?;
        index.tokenizers().register(
            CANG_JIE,
            CangJieTokenizer {
                worker: Arc::new(Jieba::empty()), // empty dictionary
                option: TokenizerOption::Unicode,
            },
        ); // Build cang-jie Tokenizer

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        Ok(DictIndex {
            index,
            schema,
            reader,
        })
    }
}
