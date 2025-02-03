use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3309875328: FileFormat = FileFormat {
    id: 3_309_875_328,
    source_type: SourceType::Iana,
    name: "G726-40",
    extensions: &[],
    media_types: &["audio/G726-40"],
    internal_signatures: &[],
    related_formats: &[],
};
