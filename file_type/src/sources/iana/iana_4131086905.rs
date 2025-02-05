use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4131086905: FileFormat = FileFormat {
    id: 4_131_086_905,
    source_type: SourceType::Iana,
    name: "vnd.geometry-explorer",
    extensions: &[],
    media_types: &["application/vnd.geometry-explorer"],
    signatures: &[],
    related_formats: &[],
};
