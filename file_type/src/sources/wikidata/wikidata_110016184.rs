use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110016184: FileType = FileType {
    file_format: &FileFormat {
        id: 110_016_184,
        source_type: SourceType::Wikidata,
        name: "Archimedes Tracker Module",
        extensions: &["musx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
