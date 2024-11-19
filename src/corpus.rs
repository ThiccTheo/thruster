use {
    super::document::Document,
    std::{
        io::{Error as IoError, Result as IoResult},
        path::{Path, PathBuf},
        slice::Iter,
    },
};

#[derive(Debug)]
pub struct Corpus(Vec<Document>);

impl Corpus {
    pub fn idf(&self, term: &str) -> f32 {
        ((1. + self.0.len() as f32)
            / (1.
                + self
                    .0
                    .iter()
                    .filter(|document| document.tf(term) > 0)
                    .count() as f32))
            .log10()
    }

    pub fn iter(&self) -> Iter<'_, Document> {
        self.0.iter()
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

impl TryFrom<&[PathBuf]> for Corpus {
    type Error = IoError;

    fn try_from(paths: &[PathBuf]) -> Result<Self, Self::Error> {
        let mut documents = vec![];
        for path in paths {
            documents.extend(Self::try_from(path.as_path())?.0);
        }
        Ok(Corpus(documents))
    }
}
