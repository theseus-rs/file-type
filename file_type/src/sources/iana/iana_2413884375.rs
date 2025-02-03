use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2413884375: FileFormat = FileFormat {
    id: 2_413_884_375,
    source_type: SourceType::Iana,
    name: "vnd.uvvu.mp4",
    extensions: &[],
    media_types: &["video/vnd.uvvu.mp4"],
    internal_signatures: &[],
    related_formats: &[],
};
