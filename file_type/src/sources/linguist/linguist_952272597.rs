use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_952272597: FileType = FileType {
    file_format: &FileFormat {
        id: 952_272_597,
        source_type: SourceType::Linguist,
        name: "TypeSpec",
        extensions: &["tsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
