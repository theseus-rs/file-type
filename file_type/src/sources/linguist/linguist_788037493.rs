use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_788037493: FileType = FileType {
    file_format: &FileFormat {
        id: 788_037_493,
        source_type: SourceType::Linguist,
        name: "Cooklang",
        extensions: &["cook"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
