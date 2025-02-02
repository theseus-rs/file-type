use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_199: FileFormat = FileFormat {
    id: 199,
    source_type: SourceType::Linguist,
    name: "Lex",
    extensions: &["l", "lex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
