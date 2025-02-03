use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3328258557: FileFormat = FileFormat {
    id: 3_328_258_557,
    source_type: SourceType::Iana,
    name: "mp4",
    extensions: &[],
    media_types: &["audio/mp4"],
    internal_signatures: &[],
    related_formats: &[],
};
