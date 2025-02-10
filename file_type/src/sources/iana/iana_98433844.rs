use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_98433844: FileType = FileType {
    file_format: &FileFormat {
        id: 98_433_844,
        source_type: SourceType::Iana,
        name: "mbms-deregister+xml",
        extensions: &[],
        media_types: &["application/mbms-deregister+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
