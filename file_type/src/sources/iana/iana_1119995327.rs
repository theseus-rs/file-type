use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1119995327: FileType = FileType {
    file_format: &FileFormat {
        id: 1_119_995_327,
        source_type: SourceType::Iana,
        name: "vnd.vel+json",
        extensions: &[],
        media_types: &["application/vnd.vel+json"],
        signatures: &[],
        related_formats: &[],
    },
};
