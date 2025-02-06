use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_398300339: FileFormat = FileFormat {
    id: 398_300_339,
    source_type: SourceType::Iana,
    name: "vnd.hns.audio",
    extensions: &[],
    media_types: &["audio/vnd.hns.audio"],
    signatures: &[],
    related_formats: &[],
};
