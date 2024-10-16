use {
    super::query::Query,
    scraper::{Html, Selector},
    std::{collections::HashMap, fs, io::Error as IoError, path::Path},
};

#[derive(Debug)]
pub struct Document {
    term_to_count: HashMap<String, u32>,
}

impl Document {
    pub fn tf(&self, term: &str) -> u32 {
        self.term_to_count
            .get(&term.to_owned())
            .copied()
            .unwrap_or_default()
    }
}

impl TryFrom<&Path> for Document {
    type Error = IoError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut doc = Document {
            term_to_count: HashMap::default(),
        };

        Html::parse_document(&fs::read_to_string(path)?)
            .select(&Selector::parse("body").unwrap())
            .next()
            .unwrap()
            .text()
            .flat_map(|terms| terms.split_whitespace())
            .map(|term| term.to_lowercase())
            .filter(|term| !Query::STOP_WORDS.contains(&term.as_str()))
            .for_each(|term| *doc.term_to_count.entry(term).or_default() += 1);

        Ok(doc)
    }
}
