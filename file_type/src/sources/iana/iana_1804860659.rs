use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1804860659: FileType = FileType {
    file_format: &FileFormat {
        id: 1_804_860_659,
        source_type: SourceType::Iana,
        name: "vnd.ms-cab-compressed",
        extensions: &[],
        media_types: &["application/vnd.ms-cab-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
