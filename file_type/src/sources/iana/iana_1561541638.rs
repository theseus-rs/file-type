use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1561541638: FileFormat = FileFormat {
    id: 1_561_541_638,
    source_type: SourceType::Iana,
    name: "vnd.in3d.3dml",
    extensions: &[],
    media_types: &["text/vnd.in3d.3dml"],
    internal_signatures: &[],
    related_formats: &[],
};
