use {
    super::query::Query, scraper::{Html, Selector}, std::{
        collections::HashMap,
        fs,
        io::Error as IoError,
        path::{Path, PathBuf},
    },
};

#[derive(Debug)]
pub struct Document {
    path: PathBuf,
    term_to_count: HashMap<String, u32>,
}

impl Document {
    pub fn tf(&self, term: &str) -> u32 {
        self.term_to_count.get(term).copied().unwrap_or_default()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl TryFrom<&Path> for Document {
    type Error = IoError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut document = Document {
            path: path.to_path_buf(),
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
            .for_each(|term| *document.term_to_count.entry(term).or_default() += 1);

        Ok(document)
    }
}
