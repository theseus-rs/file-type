use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3757765885: FileFormat = FileFormat {
    id: 3_757_765_885,
    source_type: SourceType::Iana,
    name: "vnd.flatland.3dml",
    extensions: &[],
    media_types: &["model/vnd.flatland.3dml"],
    signatures: &[],
    related_formats: &[],
};
