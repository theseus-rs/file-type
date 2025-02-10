use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_227: FileType = FileType {
    file_format: &FileFormat {
        id: 227,
        source_type: SourceType::Linguist,
        name: "Max",
        extensions: &["maxhelp", "maxpat", "maxproj", "mxt", "pat"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
