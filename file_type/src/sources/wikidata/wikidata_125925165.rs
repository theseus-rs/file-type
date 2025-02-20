use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125925165: FileType = FileType {
    file_format: &FileFormat {
        id: 125_925_165,
        source_type: SourceType::Wikidata,
        name: "Papyrus Base Formula file",
        extensions: &["pbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
