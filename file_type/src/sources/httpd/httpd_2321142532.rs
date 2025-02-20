use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2321142532: FileType = FileType {
    file_format: &FileFormat {
        id: 2_321_142_532,
        source_type: SourceType::Httpd,
        name: "dreamfactory",
        extensions: &["dfac"],
        media_types: &["application/vnd.dreamfactory"],
        signatures: &[],
        related_formats: &[],
    },
};
