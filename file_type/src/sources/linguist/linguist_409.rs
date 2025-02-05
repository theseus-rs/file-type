use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_409: FileFormat = FileFormat {
    id: 409,
    source_type: SourceType::Linguist,
    name: "Yacc",
    extensions: &["y", "yacc", "yy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
