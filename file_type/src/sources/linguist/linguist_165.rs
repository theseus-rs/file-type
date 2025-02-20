use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_165: FileType = FileType {
    file_format: &FileFormat {
        id: 165,
        source_type: SourceType::Linguist,
        name: "Idris",
        extensions: &["idr", "lidr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
