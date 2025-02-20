use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28106114: FileType = FileType {
    file_format: &FileFormat {
        id: 28_106_114,
        source_type: SourceType::Wikidata,
        name: "GRASP font",
        extensions: &["fnt", "set"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
