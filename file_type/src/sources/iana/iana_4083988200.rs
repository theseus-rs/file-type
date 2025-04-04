use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4083988200: FileType = FileType {
    file_format: &FileFormat {
        id: 4_083_988_200,
        source_type: SourceType::Iana,
        name: "raw",
        extensions: &[],
        media_types: &["video/raw"],
        signatures: &[],
        related_formats: &[],
    },
};
