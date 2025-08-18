use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137496: FileType = FileType {
    file_format: &FileFormat {
        id: 137_496,
        source_type: SourceType::Wikidata,
        name: "Java bytecode",
        extensions: &["class"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
