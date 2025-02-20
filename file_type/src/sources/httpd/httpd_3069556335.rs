use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3069556335: FileType = FileType {
    file_format: &FileFormat {
        id: 3_069_556_335,
        source_type: SourceType::Httpd,
        name: "sun xml math",
        extensions: &["sxm"],
        media_types: &["application/vnd.sun.xml.math"],
        signatures: &[],
        related_formats: &[],
    },
};
