use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2452209651: FileType = FileType {
    file_format: &FileFormat {
        id: 2_452_209_651,
        source_type: SourceType::Httpd,
        name: "geospace",
        extensions: &["g3w"],
        media_types: &["application/vnd.geospace"],
        signatures: &[],
        related_formats: &[],
    },
};
