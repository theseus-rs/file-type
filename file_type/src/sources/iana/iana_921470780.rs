use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_921470780: FileType = FileType {
    file_format: &FileFormat {
        id: 921_470_780,
        source_type: SourceType::Iana,
        name: "v3c",
        extensions: &[],
        media_types: &["application/v3c"],
        signatures: &[],
        related_formats: &[],
    },
};
