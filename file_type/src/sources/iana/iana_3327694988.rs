use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3327694988: FileType = FileType {
    file_format: &FileFormat {
        id: 3_327_694_988,
        source_type: SourceType::Iana,
        name: "vnd.sealed.swf",
        extensions: &[],
        media_types: &["video/vnd.sealed.swf"],
        signatures: &[],
        related_formats: &[],
    },
};
