use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1105824498: FileFormat = FileFormat {
    id: 1_105_824_498,
    source_type: SourceType::Iana,
    name: "3gpp-ims+xml",
    extensions: &[],
    media_types: &["application/3gpp-ims+xml"],
    signatures: &[],
    related_formats: &[],
};
