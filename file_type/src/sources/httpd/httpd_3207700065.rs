use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3207700065: FileType = FileType {
    file_format: &FileFormat {
        id: 3_207_700_065,
        source_type: SourceType::Httpd,
        name: "symbian install",
        extensions: &["sis", "sisx"],
        media_types: &["application/vnd.symbian.install"],
        signatures: &[],
        related_formats: &[],
    },
};
