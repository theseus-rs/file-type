use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27925699: FileType = FileType {
    file_format: &FileFormat {
        id: 27_925_699,
        source_type: SourceType::Wikidata,
        name: "DTED Level 0 Average Terrain Elevation Value File",
        extensions: &["avg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
