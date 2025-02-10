use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206306: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_306,
        source_type: SourceType::Wikidata,
        name: "Analyze IMG",
        extensions: &["img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
