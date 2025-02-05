use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3988587278: FileFormat = FileFormat {
    id: 3_988_587_278,
    source_type: SourceType::Iana,
    name: "vnd.sealed.mpeg4",
    extensions: &[],
    media_types: &["video/vnd.sealed.mpeg4"],
    signatures: &[],
    related_formats: &[],
};
