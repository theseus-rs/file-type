use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_55627273: FileType = FileType {
    file_format: &FileFormat {
        id: 55_627_273,
        source_type: SourceType::Linguist,
        name: "Carbon",
        extensions: &["carbon"],
        media_types: &["text/x-go"],
        signatures: &[],
        related_formats: &[],
    },
};
