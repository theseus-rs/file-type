use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1414658833: FileType = FileType {
    file_format: &FileFormat {
        id: 1_414_658_833,
        source_type: SourceType::Httpd,
        name: "blorb",
        extensions: &["blb", "blorb"],
        media_types: &["application/x-blorb"],
        signatures: &[],
        related_formats: &[],
    },
};
