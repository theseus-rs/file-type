use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1151597811: FileType = FileType {
    file_format: &FileFormat {
        id: 1_151_597_811,
        source_type: SourceType::Httpd,
        name: "ktx",
        extensions: &["ktx"],
        media_types: &["image/ktx"],
        signatures: &[],
        related_formats: &[],
    },
};
