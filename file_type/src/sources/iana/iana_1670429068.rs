use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1670429068: FileType = FileType {
    file_format: &FileFormat {
        id: 1_670_429_068,
        source_type: SourceType::Iana,
        name: "iges",
        extensions: &[],
        media_types: &["model/iges"],
        signatures: &[],
        related_formats: &[],
    },
};
