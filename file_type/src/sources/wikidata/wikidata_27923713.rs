use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27923713: FileType = FileType {
    file_format: &FileFormat {
        id: 27_923_713,
        source_type: SourceType::Wikidata,
        name: "DTED Level 1 Terrain Elevation Data File",
        extensions: &["dt1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
