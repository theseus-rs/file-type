use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_396279391: FileType = FileType {
    file_format: &FileFormat {
        id: 396_279_391,
        source_type: SourceType::Iana,
        name: "vnd.collection.next+json",
        extensions: &[],
        media_types: &["application/vnd.collection.next+json"],
        signatures: &[],
        related_formats: &[],
    },
};
