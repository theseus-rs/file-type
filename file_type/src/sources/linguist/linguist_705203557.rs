use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_705203557: FileType = FileType {
    file_format: &FileFormat {
        id: 705_203_557,
        source_type: SourceType::Linguist,
        name: "crontab",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
