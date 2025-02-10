use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2522134023: FileType = FileType {
    file_format: &FileFormat {
        id: 2_522_134_023,
        source_type: SourceType::Httpd,
        name: "pics rules",
        extensions: &["prf"],
        media_types: &["application/pics-rules"],
        signatures: &[],
        related_formats: &[],
    },
};
