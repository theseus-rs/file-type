use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2248450100: FileType = FileType {
    file_format: &FileFormat {
        id: 2_248_450_100,
        source_type: SourceType::Httpd,
        name: "3m post it notes",
        extensions: &["pwn"],
        media_types: &["application/vnd.3m.post-it-notes"],
        signatures: &[],
        related_formats: &[],
    },
};
