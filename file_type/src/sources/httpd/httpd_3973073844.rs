use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3973073844: FileType = FileType {
    file_format: &FileFormat {
        id: 3_973_073_844,
        source_type: SourceType::Httpd,
        name: "ace compressed",
        extensions: &["ace"],
        media_types: &["application/x-ace-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
