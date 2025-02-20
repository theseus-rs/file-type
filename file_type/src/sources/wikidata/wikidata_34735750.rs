use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34735750: FileType = FileType {
    file_format: &FileFormat {
        id: 34_735_750,
        source_type: SourceType::Wikidata,
        name: "SimTower saved game",
        extensions: &["tdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
