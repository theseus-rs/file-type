use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_939688699: FileType = FileType {
    file_format: &FileFormat {
        id: 939_688_699,
        source_type: SourceType::Iana,
        name: "ogg",
        extensions: &[],
        media_types: &["video/ogg"],
        signatures: &[],
        related_formats: &[],
    },
};
