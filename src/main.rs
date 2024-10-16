mod corpus;
mod document;
mod query;

use {corpus::Corpus, std::path::PathBuf, query::Query};

fn main() {
    let corpus = Corpus::try_from(
        PathBuf::from(format!(
            "{}/java/net",
            env!("CARGO_MANIFEST_DIR")
        ))
        .as_path(),
    )
    .unwrap();

    let qry = Query::from("data structure insert algorithm time variable parameter");
    let res = qry.search(&corpus);
    println!("{:?}", res);
}