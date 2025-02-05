use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2413884375: FileFormat = FileFormat {
    id: 2_413_884_375,
    source_type: SourceType::Httpd,
    name: "uvvu mp4",
    extensions: &["uvu", "uvvu"],
    media_types: &["video/vnd.uvvu.mp4"],
    signatures: &[],
    related_formats: &[],
};
