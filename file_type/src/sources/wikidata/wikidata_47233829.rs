use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47233829: FileType = FileType {
    file_format: &FileFormat {
        id: 47_233_829,
        source_type: SourceType::Wikidata,
        name: "L3B",
        extensions: &["l3b"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
