use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3086374288: FileType = FileType {
    file_format: &FileFormat {
        id: 3_086_374_288,
        source_type: SourceType::Httpd,
        name: "dece graphic",
        extensions: &["uvi", "uvvi", "uvg", "uvvg"],
        media_types: &["image/vnd.dece.graphic"],
        signatures: &[],
        related_formats: &[],
    },
};
