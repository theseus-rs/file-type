use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4129216099: FileType = FileType {
    file_format: &FileFormat {
        id: 4_129_216_099,
        source_type: SourceType::Httpd,
        name: "jpeg",
        extensions: &["jpgv"],
        media_types: &["video/jpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
