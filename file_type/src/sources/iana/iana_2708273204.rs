use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2708273204: FileType = FileType {
    file_format: &FileFormat {
        id: 2_708_273_204,
        source_type: SourceType::Iana,
        name: "vnd.uic.osdm+json",
        extensions: &[],
        media_types: &["application/vnd.uic.osdm+json"],
        signatures: &[],
        related_formats: &[],
    },
};
