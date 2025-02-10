use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_792408528: FileType = FileType {
    file_format: &FileFormat {
        id: 792_408_528,
        source_type: SourceType::Linguist,
        name: "Genie",
        extensions: &["gs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
