use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_89855901: FileFormat = FileFormat {
    id: 89_855_901,
    source_type: SourceType::Linguist,
    name: "StringTemplate",
    extensions: &["st"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
