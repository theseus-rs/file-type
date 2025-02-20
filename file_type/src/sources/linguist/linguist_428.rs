use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_428: FileType = FileType {
    file_format: &FileFormat {
        id: 428,
        source_type: SourceType::Linguist,
        name: "Python console",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
