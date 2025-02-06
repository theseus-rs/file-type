use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_237: FileFormat = FileFormat {
    id: 237,
    source_type: SourceType::Linguist,
    name: "Moocode",
    extensions: &["moo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
