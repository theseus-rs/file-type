use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3497392718: FileType = FileType {
    file_format: &FileFormat {
        id: 3_497_392_718,
        source_type: SourceType::Httpd,
        name: "sun xml writer template",
        extensions: &["stw"],
        media_types: &["application/vnd.sun.xml.writer.template"],
        signatures: &[],
        related_formats: &[],
    },
};
