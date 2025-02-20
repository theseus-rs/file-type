use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_366607477: FileType = FileType {
    file_format: &FileFormat {
        id: 366_607_477,
        source_type: SourceType::Linguist,
        name: "HAProxy",
        extensions: &["cfg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
