use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1252036055: FileType = FileType {
    file_format: &FileFormat {
        id: 1_252_036_055,
        source_type: SourceType::Httpd,
        name: "ms word template macroenabled 12",
        extensions: &["dotm"],
        media_types: &["application/vnd.ms-word.template.macroenabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
