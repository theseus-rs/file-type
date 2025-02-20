use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_214: FileType = FileType {
    file_format: &FileFormat {
        id: 214,
        source_type: SourceType::Linguist,
        name: "M",
        extensions: &["m", "mumps"],
        media_types: &["text/x-mumps"],
        signatures: &[],
        related_formats: &[],
    },
};
