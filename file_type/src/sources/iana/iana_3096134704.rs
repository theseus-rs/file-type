use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3096134704: FileType = FileType {
    file_format: &FileFormat {
        id: 3_096_134_704,
        source_type: SourceType::Iana,
        name: "vtt",
        extensions: &[],
        media_types: &["text/vtt"],
        signatures: &[],
        related_formats: &[],
    },
};
