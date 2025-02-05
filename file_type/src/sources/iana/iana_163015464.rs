use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_163015464: FileFormat = FileFormat {
    id: 163_015_464,
    source_type: SourceType::Iana,
    name: "ibe-key-request+xml",
    extensions: &[],
    media_types: &["application/ibe-key-request+xml"],
    signatures: &[],
    related_formats: &[],
};
