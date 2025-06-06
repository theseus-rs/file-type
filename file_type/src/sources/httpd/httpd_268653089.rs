use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_268653089: FileType = FileType {
    file_format: &FileFormat {
        id: 268_653_089,
        source_type: SourceType::Httpd,
        name: "oasis opendocument text web",
        extensions: &["oth"],
        media_types: &["application/vnd.oasis.opendocument.text-web"],
        signatures: &[],
        related_formats: &[],
    },
};
