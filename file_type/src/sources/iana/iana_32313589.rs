use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_32313589: FileType = FileType {
    file_format: &FileFormat {
        id: 32_313_589,
        source_type: SourceType::Iana,
        name: "vnd.artisan+json",
        extensions: &[],
        media_types: &["application/vnd.artisan+json"],
        signatures: &[],
        related_formats: &[],
    },
};
