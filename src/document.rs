use {
    super::query::Query,
    scraper::{Html, Selector},
    std::{
        collections::HashMap,
        fs,
        io::Error as IoError,
        path::{Path, PathBuf},
    },
};

#[derive(Debug)]
pub struct Document {
    path: PathBuf,
    title: String,
    term_to_count: HashMap<String, u32>,
}

impl Document {
    pub fn tf(&self, term: &str) -> u32 {
        self.term_to_count.get(term).copied().unwrap_or_default()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

impl TryFrom<&Path> for Document {
    type Error = IoError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let html = Html::parse_document(&fs::read_to_string(path)?);

        let title = html
            .select(&Selector::parse("title").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_owned();

        let mut document = Document {
            path: path.to_path_buf(),
            title,
            term_to_count: HashMap::default(),
        };

        html.select(&Selector::parse("body").unwrap())
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
