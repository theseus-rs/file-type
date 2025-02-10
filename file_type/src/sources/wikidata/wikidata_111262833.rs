use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111262833: FileType = FileType {
    file_format: &FileFormat {
        id: 111_262_833,
        source_type: SourceType::Wikidata,
        name: "Velvet Studio instrument",
        extensions: &["ais"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
