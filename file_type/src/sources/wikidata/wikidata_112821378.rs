use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112821378: FileType = FileType {
    file_format: &FileFormat {
        id: 112_821_378,
        source_type: SourceType::Wikidata,
        name: "Minolta 3D Scanner Camera File",
        extensions: &["cam"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
