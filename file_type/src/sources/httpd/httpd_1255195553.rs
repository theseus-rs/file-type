use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1255195553: FileType = FileType {
    file_format: &FileFormat {
        id: 1_255_195_553,
        source_type: SourceType::Httpd,
        name: "3gpp",
        extensions: &["3gp"],
        media_types: &["video/3gpp"],
        signatures: &[],
        related_formats: &[],
    },
};
