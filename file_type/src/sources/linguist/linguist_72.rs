use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_72: FileFormat = FileFormat {
    id: 72,
    source_type: SourceType::Linguist,
    name: "Crystal",
    extensions: &["cr"],
    media_types: &["text/x-crystal"],
    internal_signatures: &[],
    related_formats: &[],
};
