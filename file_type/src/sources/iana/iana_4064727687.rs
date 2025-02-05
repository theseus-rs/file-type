use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4064727687: FileFormat = FileFormat {
    id: 4_064_727_687,
    source_type: SourceType::Iana,
    name: "vnd.cryptomator.encrypted",
    extensions: &[],
    media_types: &["application/vnd.cryptomator.encrypted"],
    signatures: &[],
    related_formats: &[],
};
