use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27923715: FileType = FileType {
    file_format: &FileFormat {
        id: 27_923_715,
        source_type: SourceType::Wikidata,
        name: "DTED Level 2 Terrain Elevation Data File",
        extensions: &["dt2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
