use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125255959: FileType = FileType {
    file_format: &FileFormat {
        id: 125_255_959,
        source_type: SourceType::Wikidata,
        name: "Simulation Settings File",
        extensions: &["sim"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
