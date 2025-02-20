use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129494019: FileType = FileType {
    file_format: &FileFormat {
        id: 129_494_019,
        source_type: SourceType::Wikidata,
        name: "GSQL query file",
        extensions: &["gsql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
