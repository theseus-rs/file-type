use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_533974994: FileFormat = FileFormat {
    id: 533_974_994,
    source_type: SourceType::Iana,
    name: "vnd.oma.pal+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.pal+xml"],
    signatures: &[],
    related_formats: &[],
};
