use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3640433183: FileType = FileType {
    file_format: &FileFormat {
        id: 3_640_433_183,
        source_type: SourceType::Httpd,
        name: "scvp vp request",
        extensions: &["spq"],
        media_types: &["application/scvp-vp-request"],
        signatures: &[],
        related_formats: &[],
    },
};
