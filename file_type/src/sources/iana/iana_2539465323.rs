use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2539465323: FileType = FileType {
    file_format: &FileFormat {
        id: 2_539_465_323,
        source_type: SourceType::Iana,
        name: "png",
        extensions: &[],
        media_types: &["image/png"],
        signatures: &[],
        related_formats: &[],
    },
};
