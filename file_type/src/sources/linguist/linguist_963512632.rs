use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_963512632: FileType = FileType {
    file_format: &FileFormat {
        id: 963_512_632,
        source_type: SourceType::Linguist,
        name: "Earthly",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
