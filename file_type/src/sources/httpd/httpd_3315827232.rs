use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3315827232: FileType = FileType {
    file_format: &FileFormat {
        id: 3_315_827_232,
        source_type: SourceType::Httpd,
        name: "oasis opendocument text template",
        extensions: &["ott"],
        media_types: &["application/vnd.oasis.opendocument.text-template"],
        signatures: &[],
        related_formats: &[],
    },
};
