use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1804860659: FileType = FileType {
    file_format: &FileFormat {
        id: 1_804_860_659,
        source_type: SourceType::Httpd,
        name: "ms cab compressed",
        extensions: &["cab"],
        media_types: &["application/vnd.ms-cab-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
