use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_97358117: FileType = FileType {
    file_format: &FileFormat {
        id: 97_358_117,
        source_type: SourceType::Linguist,
        name: "Futhark",
        extensions: &["fut"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
