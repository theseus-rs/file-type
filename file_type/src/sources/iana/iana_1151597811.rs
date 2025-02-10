use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1151597811: FileType = FileType {
    file_format: &FileFormat {
        id: 1_151_597_811,
        source_type: SourceType::Iana,
        name: "ktx",
        extensions: &[],
        media_types: &["image/ktx"],
        signatures: &[],
        related_formats: &[],
    },
};
