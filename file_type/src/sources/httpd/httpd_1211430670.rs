use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1211430670: FileType = FileType {
    file_format: &FileFormat {
        id: 1_211_430_670,
        source_type: SourceType::Httpd,
        name: "oasis opendocument text",
        extensions: &["odt"],
        media_types: &["application/vnd.oasis.opendocument.text"],
        signatures: &[],
        related_formats: &[],
    },
};
