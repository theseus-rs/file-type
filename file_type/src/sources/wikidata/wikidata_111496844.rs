use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111496844: FileType = FileType {
    file_format: &FileFormat {
        id: 111_496_844,
        source_type: SourceType::Wikidata,
        name: "SPYne Containers",
        extensions: &["spy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
