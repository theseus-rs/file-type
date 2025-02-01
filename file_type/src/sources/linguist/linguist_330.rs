use crate::format::FileFormat;

pub(crate) const LINGUIST_330: FileFormat = FileFormat {
    id: 330,
    puid: "linguist/330",
    name: "SMT",
    extensions: &["smt", "smt2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
