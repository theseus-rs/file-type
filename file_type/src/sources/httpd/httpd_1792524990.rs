use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1792524990: FileType = FileType {
    file_format: &FileFormat {
        id: 1_792_524_990,
        source_type: SourceType::Httpd,
        name: "antix game component",
        extensions: &["atx"],
        media_types: &["application/vnd.antix.game-component"],
        signatures: &[],
        related_formats: &[],
    },
};
