use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6912474: FileType = FileType {
    file_format: &FileFormat {
        id: 6_912_474,
        source_type: SourceType::Wikidata,
        name: "Mork",
        extensions: &["dat", "mab", "msf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
