use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27925703: FileType = FileType {
    file_format: &FileFormat {
        id: 27_925_703,
        source_type: SourceType::Wikidata,
        name: "DTED Level 0 Maximum Terrain Elevation Value File",
        extensions: &["max"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
