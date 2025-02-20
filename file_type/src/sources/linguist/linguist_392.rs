use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_392: FileType = FileType {
    file_format: &FileFormat {
        id: 392,
        source_type: SourceType::Linguist,
        name: "Wavefront Material",
        extensions: &["mtl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
