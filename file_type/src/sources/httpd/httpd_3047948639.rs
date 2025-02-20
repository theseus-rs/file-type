use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3047948639: FileType = FileType {
    file_format: &FileFormat {
        id: 3_047_948_639,
        source_type: SourceType::Httpd,
        name: "bmi",
        extensions: &["bmi"],
        media_types: &["application/vnd.bmi"],
        signatures: &[],
        related_formats: &[],
    },
};
