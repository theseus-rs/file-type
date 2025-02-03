use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3560121387: FileFormat = FileFormat {
    id: 3_560_121_387,
    source_type: SourceType::Iana,
    name: "L16",
    extensions: &[],
    media_types: &["audio/L16"],
    internal_signatures: &[],
    related_formats: &[],
};
