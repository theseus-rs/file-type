use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3449934334: FileType = FileType {
    file_format: &FileFormat {
        id: 3_449_934_334,
        source_type: SourceType::Httpd,
        name: "shana informed formtemplate",
        extensions: &["itp"],
        media_types: &["application/vnd.shana.informed.formtemplate"],
        signatures: &[],
        related_formats: &[],
    },
};
