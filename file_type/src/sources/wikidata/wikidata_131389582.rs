use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131389582: FileType = FileType {
    file_format: &FileFormat {
        id: 131_389_582,
        source_type: SourceType::Wikidata,
        name: "Velocity file format",
        extensions: &["vm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
