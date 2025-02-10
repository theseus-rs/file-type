use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_171816475: FileType = FileType {
    file_format: &FileFormat {
        id: 171_816_475,
        source_type: SourceType::Httpd,
        name: "lucent voice",
        extensions: &["lvp"],
        media_types: &["audio/vnd.lucent.voice"],
        signatures: &[],
        related_formats: &[],
    },
};
