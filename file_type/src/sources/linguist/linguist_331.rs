use crate::format::FileFormat;

pub(crate) const LINGUIST_331: FileFormat = FileFormat {
    id: 331,
    puid: "linguist/331",
    name: "SPARQL",
    extensions: &["rq", "sparql"],
    media_types: &["application/sparql-query"],
    internal_signatures: &[],
    related_formats: &[],
};
