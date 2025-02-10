use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_855119395: FileType = FileType {
    file_format: &FileFormat {
        id: 855_119_395,
        source_type: SourceType::Httpd,
        name: "uiq theme",
        extensions: &["utz"],
        media_types: &["application/vnd.uiq.theme"],
        signatures: &[],
        related_formats: &[],
    },
};
