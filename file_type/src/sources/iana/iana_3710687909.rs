use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3710687909: FileType = FileType {
    file_format: &FileFormat {
        id: 3_710_687_909,
        source_type: SourceType::Iana,
        name: "vnd.crick.clicker.template",
        extensions: &[],
        media_types: &["application/vnd.crick.clicker.template"],
        signatures: &[],
        related_formats: &[],
    },
};
