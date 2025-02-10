use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1236131182: FileType = FileType {
    file_format: &FileFormat {
        id: 1_236_131_182,
        source_type: SourceType::Httpd,
        name: "joost joda archive",
        extensions: &["joda"],
        media_types: &["application/vnd.joost.joda-archive"],
        signatures: &[],
        related_formats: &[],
    },
};
