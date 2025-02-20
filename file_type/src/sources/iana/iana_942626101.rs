use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_942626101: FileType = FileType {
    file_format: &FileFormat {
        id: 942_626_101,
        source_type: SourceType::Iana,
        name: "vnd.ms-3mfdocument",
        extensions: &[],
        media_types: &["application/vnd.ms-3mfdocument"],
        signatures: &[],
        related_formats: &[],
    },
};
