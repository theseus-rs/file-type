use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_235: FileFormat = FileFormat {
    id: 235,
    source_type: SourceType::Linguist,
    name: "Module Management System",
    extensions: &["mmk", "mms"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
