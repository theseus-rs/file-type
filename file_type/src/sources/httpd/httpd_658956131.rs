use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_658956131: FileType = FileType {
    file_format: &FileFormat {
        id: 658_956_131,
        source_type: SourceType::Httpd,
        name: "sun xml impress template",
        extensions: &["sti"],
        media_types: &["application/vnd.sun.xml.impress.template"],
        signatures: &[],
        related_formats: &[],
    },
};
