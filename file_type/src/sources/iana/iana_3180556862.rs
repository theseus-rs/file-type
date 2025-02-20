use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3180556862: FileType = FileType {
    file_format: &FileFormat {
        id: 3_180_556_862,
        source_type: SourceType::Iana,
        name: "vnd.yamaha.through-ngn",
        extensions: &[],
        media_types: &["application/vnd.yamaha.through-ngn"],
        signatures: &[],
        related_formats: &[],
    },
};
