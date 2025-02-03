use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_758480799: FileFormat = FileFormat {
    id: 758_480_799,
    source_type: SourceType::Linguist,
    name: "Lark",
    extensions: &["lark"],
    media_types: &["text/x-ebnf"],
    internal_signatures: &[],
    related_formats: &[],
};
