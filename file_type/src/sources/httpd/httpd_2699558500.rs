use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2699558500: FileType = FileType {
    file_format: &FileFormat {
        id: 2_699_558_500,
        source_type: SourceType::Httpd,
        name: "hp hps",
        extensions: &["hps"],
        media_types: &["application/vnd.hp-hps"],
        signatures: &[],
        related_formats: &[],
    },
};
