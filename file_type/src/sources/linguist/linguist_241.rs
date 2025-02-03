use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_241: FileFormat = FileFormat {
    id: 241,
    source_type: SourceType::Linguist,
    name: "NL",
    extensions: &["nl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
