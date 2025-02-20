use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2904866414: FileType = FileType {
    file_format: &FileFormat {
        id: 2_904_866_414,
        source_type: SourceType::Httpd,
        name: "jisp",
        extensions: &["jisp"],
        media_types: &["application/vnd.jisp"],
        signatures: &[],
        related_formats: &[],
    },
};
