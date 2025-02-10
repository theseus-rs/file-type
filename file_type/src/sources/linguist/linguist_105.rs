use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_105: FileType = FileType {
    file_format: &FileFormat {
        id: 105,
        source_type: SourceType::Linguist,
        name: "F#",
        extensions: &["fs", "fsi", "fsx"],
        media_types: &["text/x-fsharp"],
        signatures: &[],
        related_formats: &[],
    },
};
