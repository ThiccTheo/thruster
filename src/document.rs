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

#[derive(Debug, Clone)]
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
        path.extension()
            .is_some_and(|extension| extension == "html")
            .then_some(())
            .ok_or(IoError::other("invalid file extension, must be .html"))?;

        let html = Html::parse_document(&fs::read_to_string(path)?);

        let title = html
            .select(&Selector::parse("title").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_owned();

        let mut term_to_count = HashMap::default();

        html.select(&Selector::parse("body").unwrap())
            .next()
            .unwrap()
            .text()
            .flat_map(|terms| terms.split_whitespace())
            .map(|term| term.to_lowercase())
            .filter(|term| !Query::STOP_WORDS.contains(&term.as_str()))
            .for_each(|term| *term_to_count.entry(term).or_default() += 1);

        Ok(Document {
            path: path.to_owned(),
            title,
            term_to_count,
        })
    }
}
