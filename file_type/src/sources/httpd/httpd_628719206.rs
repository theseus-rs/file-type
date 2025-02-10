use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_628719206: FileType = FileType {
    file_format: &FileFormat {
        id: 628_719_206,
        source_type: SourceType::Httpd,
        name: "tiff",
        extensions: &["tiff", "tif"],
        media_types: &["image/tiff"],
        signatures: &[],
        related_formats: &[],
    },
};
