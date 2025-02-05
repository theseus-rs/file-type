use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1242535117: FileFormat = FileFormat {
    id: 1_242_535_117,
    source_type: SourceType::Iana,
    name: "vnd.siren+json",
    extensions: &[],
    media_types: &["application/vnd.siren+json"],
    signatures: &[],
    related_formats: &[],
};
