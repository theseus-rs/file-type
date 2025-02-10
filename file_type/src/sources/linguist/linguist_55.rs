use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_55: FileType = FileType {
    file_format: &FileFormat {
        id: 55,
        source_type: SourceType::Linguist,
        name: "Chapel",
        extensions: &["chpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
