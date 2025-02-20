use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125298468: FileType = FileType {
    file_format: &FileFormat {
        id: 125_298_468,
        source_type: SourceType::Wikidata,
        name: "Scribe",
        extensions: &["scr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
