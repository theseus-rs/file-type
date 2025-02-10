use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1071533059: FileType = FileType {
    file_format: &FileFormat {
        id: 1_071_533_059,
        source_type: SourceType::Httpd,
        name: "stepmania package",
        extensions: &["smzip"],
        media_types: &["application/vnd.stepmania.package"],
        signatures: &[],
        related_formats: &[],
    },
};
