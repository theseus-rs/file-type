use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125703914: FileType = FileType {
    file_format: &FileFormat {
        id: 125_703_914,
        source_type: SourceType::Wikidata,
        name: "StarWriter Graphics Format",
        extensions: &["sgf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
