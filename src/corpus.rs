use {
    super::document::Document,
    std::{
        io::{Error as IoError, Result as IoResult},
        path::Path,
    },
};

#[derive(Debug)]
pub struct Corpus(Vec<Document>);

impl Corpus {
    pub fn idf(&self, term: &str) -> f32 {
        f32::log10(
            (1. + self.0.len() as f32)
                / (1. + self.0.iter().filter(|doc| doc.tf(term) > 0).count() as f32),
        )
    }

	pub fn docs(&self) -> &[Document] {
		&self.0
	}
}

impl TryFrom<&Path> for Corpus {
    type Error = IoError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut corpus = Corpus(vec![]);

        fn walk_dir(path: &Path, corpus: &mut Corpus) -> IoResult<()> {
            if path.is_dir() {
                for entry in path.read_dir()?.flatten() {
                    walk_dir(&entry.path(), corpus)?
                }
            } else {
                corpus.0.push(Document::try_from(path)?);
            }
            Ok(())
        }
        walk_dir(path, &mut corpus).map(|_| corpus)
    }
}
