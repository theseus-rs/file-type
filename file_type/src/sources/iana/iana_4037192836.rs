use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4037192836: FileFormat = FileFormat {
    id: 4_037_192_836,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"],
    signatures: &[],
    related_formats: &[],
};
