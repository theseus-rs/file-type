use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1533721519: FileFormat = FileFormat {
    id: 1_533_721_519,
    source_type: SourceType::Iana,
    name: "G711-0",
    extensions: &[],
    media_types: &["audio/G711-0"],
    signatures: &[],
    related_formats: &[],
};
