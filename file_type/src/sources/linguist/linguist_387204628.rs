use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_387204628: FileType = FileType {
    file_format: &FileFormat {
        id: 387_204_628,
        source_type: SourceType::Linguist,
        name: "2-Dimensional Array",
        extensions: &["2da"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
