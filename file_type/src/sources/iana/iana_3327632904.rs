use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3327632904: FileType = FileType {
    file_format: &FileFormat {
        id: 3_327_632_904,
        source_type: SourceType::Iana,
        name: "vnd.vectorworks",
        extensions: &[],
        media_types: &["application/vnd.vectorworks"],
        signatures: &[],
        related_formats: &[],
    },
};
