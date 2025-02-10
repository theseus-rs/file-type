use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2981984339: FileType = FileType {
    file_format: &FileFormat {
        id: 2_981_984_339,
        source_type: SourceType::Httpd,
        name: "powerbuilder6",
        extensions: &["pbd"],
        media_types: &["application/vnd.powerbuilder6"],
        signatures: &[],
        related_formats: &[],
    },
};
