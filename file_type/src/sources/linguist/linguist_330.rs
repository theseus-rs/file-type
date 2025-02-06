use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_330: FileFormat = FileFormat {
    id: 330,
    source_type: SourceType::Linguist,
    name: "SMT",
    extensions: &["smt", "smt2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
