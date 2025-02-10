use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2638993316: FileType = FileType {
    file_format: &FileFormat {
        id: 2_638_993_316,
        source_type: SourceType::Iana,
        name: "vnd.hp-hpid",
        extensions: &[],
        media_types: &["application/vnd.hp-hpid"],
        signatures: &[],
        related_formats: &[],
    },
};
