use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1413817025: FileFormat = FileFormat {
    id: 1_413_817_025,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.wordprocessingml.footer+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"],
    signatures: &[],
    related_formats: &[],
};
