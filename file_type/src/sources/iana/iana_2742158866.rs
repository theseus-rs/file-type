use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2742158866: FileType = FileType {
    file_format: &FileFormat {
        id: 2_742_158_866,
        source_type: SourceType::Iana,
        name: "vnd.wap.sl",
        extensions: &[],
        media_types: &["text/vnd.wap.sl"],
        signatures: &[],
        related_formats: &[],
    },
};
