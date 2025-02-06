use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3131680240: FileFormat = FileFormat {
    id: 3_131_680_240,
    source_type: SourceType::Iana,
    name: "H263",
    extensions: &[],
    media_types: &["video/H263"],
    signatures: &[],
    related_formats: &[],
};
