use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_408: FileFormat = FileFormat {
    id: 408,
    source_type: SourceType::Linguist,
    name: "YANG",
    extensions: &["yang"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
