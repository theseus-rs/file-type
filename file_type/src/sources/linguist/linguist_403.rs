use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_403: FileFormat = FileFormat {
    id: 403,
    source_type: SourceType::Linguist,
    name: "XS",
    extensions: &["xs"],
    media_types: &["text/x-csrc"],
    signatures: &[],
    related_formats: &[],
};
