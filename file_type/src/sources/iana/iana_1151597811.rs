use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
