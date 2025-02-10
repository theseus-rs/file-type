use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1719636634: FileType = FileType {
    file_format: &FileFormat {
        id: 1_719_636_634,
        source_type: SourceType::Iana,
        name: "alto-cdni+json",
        extensions: &[],
        media_types: &["application/alto-cdni+json"],
        signatures: &[],
        related_formats: &[],
    },
};
