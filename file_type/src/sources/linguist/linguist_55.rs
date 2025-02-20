use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
