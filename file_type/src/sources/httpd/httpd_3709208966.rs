use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3709208966: FileType = FileType {
    file_format: &FileFormat {
        id: 3_709_208_966,
        source_type: SourceType::Httpd,
        name: "dece pd",
        extensions: &["uvp", "uvvp"],
        media_types: &["video/vnd.dece.pd"],
        signatures: &[],
        related_formats: &[],
    },
};
