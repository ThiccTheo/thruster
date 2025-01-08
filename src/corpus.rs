use {
    super::document::Document,
    std::{
        io::{Error as IoError, Result as IoResult},
        ops::Deref,
        path::Path,
        slice::Iter,
    },
};

pub struct Corpus(Box<[Document]>);

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

    fn try_from(directory: &Path) -> Result<Self, Self::Error> {
        let mut documents = vec![];

        fn walk_directory(path: &Path, documents: &mut Vec<Document>) -> IoResult<()> {
            if path.is_dir() {
                for entry in path.read_dir()?.flatten() {
                    walk_directory(&entry.path(), documents)?
                }
            } else {
                documents.push(Document::try_from(path)?);
            }
            Ok(())
        }
        walk_directory(directory, &mut documents).map(|_| Corpus(documents.into_boxed_slice()))
    }
}

impl TryFrom<&[&Path]> for Corpus {
    type Error = IoError;

    fn try_from(directories: &[&Path]) -> Result<Self, Self::Error> {
        if directories.is_empty() {
            Err(IoError::other("no folders were selected"))
        } else {
            directories
                .into_iter()
                .map(Deref::deref)
                .map(Corpus::try_from)
                .reduce(|accumulator, corpus| {
                    accumulator.map(|accumulator| {
                        let mut accumulator_documents = accumulator.0.into_vec();
                        let corpus_documents =
                            corpus.map(|corpus| corpus.0.into_vec()).unwrap_or_default();
                        accumulator_documents.extend(corpus_documents);
                        Corpus(accumulator_documents.into_boxed_slice())
                    })
                })
                .unwrap()
        }
    }
}
