use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3148619445: FileType = FileType {
    file_format: &FileFormat {
        id: 3_148_619_445,
        source_type: SourceType::Httpd,
        name: "msclip",
        extensions: &["clp"],
        media_types: &["application/x-msclip"],
        signatures: &[],
        related_formats: &[],
    },
};
