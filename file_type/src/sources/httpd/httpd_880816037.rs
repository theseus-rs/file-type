use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_880816037: FileType = FileType {
    file_format: &FileFormat {
        id: 880_816_037,
        source_type: SourceType::Httpd,
        name: "cmu raster",
        extensions: &["ras"],
        media_types: &["image/x-cmu-raster"],
        signatures: &[],
        related_formats: &[],
    },
};
