use itertools::Itertools;

use super::{corpus::Corpus, document::Document};

pub struct Query {
    terms: Box<[Box<str>]>,
}

impl Query {
    pub fn search(self, corpus: &Corpus) -> Box<[Document]> {
        corpus
            .iter()
            .map(|document| {
                (
                    document,
                    self.terms
                        .iter()
                        .map(|term| Self::tf_idf(term, document, corpus))
                        .sum::<f32>(),
                )
            })
            .filter(|(_, score)| *score > 0.001)
            .sorted_by(|(_, score1), (_, score2)| score2.partial_cmp(score1).unwrap())
            .map(|(document, _)| document.clone())
            .collect_vec()
            .into_boxed_slice()
    }

    fn tf_idf(term: &str, document: &Document, corpus: &Corpus) -> f32 {
        document.tf(term) as f32 * corpus.idf(term)
    }
}

impl From<&str> for Query {
    fn from(query: &str) -> Self {
        Self {
            terms: query
                .split_whitespace()
                .map(str::to_lowercase)
                .map(String::into_boxed_str)
                .collect_vec()
                .into_boxed_slice(),
        }
    }
}
