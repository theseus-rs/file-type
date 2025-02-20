use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3245366757: FileType = FileType {
    file_format: &FileFormat {
        id: 3_245_366_757,
        source_type: SourceType::Iana,
        name: "vnd.sealed.mpeg1",
        extensions: &[],
        media_types: &["video/vnd.sealed.mpeg1"],
        signatures: &[],
        related_formats: &[],
    },
};
