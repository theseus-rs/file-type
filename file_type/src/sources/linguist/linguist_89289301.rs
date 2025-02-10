use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_89289301: FileType = FileType {
    file_format: &FileFormat {
        id: 89_289_301,
        source_type: SourceType::Linguist,
        name: "TSPLIB data",
        extensions: &["tsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
