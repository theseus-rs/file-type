use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132890210: FileType = FileType {
    file_format: &FileFormat {
        id: 132_890_210,
        source_type: SourceType::Wikidata,
        name: "Axon Binary Format",
        extensions: &["abf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
