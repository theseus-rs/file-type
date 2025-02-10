use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_361957890: FileType = FileType {
    file_format: &FileFormat {
        id: 361_957_890,
        source_type: SourceType::Iana,
        name: "vnd.syncml.dm+wbxml",
        extensions: &[],
        media_types: &["application/vnd.syncml.dm+wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
