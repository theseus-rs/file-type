use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_940373278: FileType = FileType {
    file_format: &FileFormat {
        id: 940_373_278,
        source_type: SourceType::Iana,
        name: "cgm",
        extensions: &[],
        media_types: &["image/cgm"],
        signatures: &[],
        related_formats: &[],
    },
};
