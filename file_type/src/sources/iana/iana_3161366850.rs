use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3161366850: FileFormat = FileFormat {
    id: 3_161_366_850,
    source_type: SourceType::Iana,
    name: "vnd.ms-lrm",
    extensions: &[],
    media_types: &["application/vnd.ms-lrm"],
    signatures: &[],
    related_formats: &[],
};
