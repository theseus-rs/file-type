use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110086833: FileType = FileType {
    file_format: &FileFormat {
        id: 110_086_833,
        source_type: SourceType::Wikidata,
        name: "Agisoft Tiled Model",
        extensions: &["tls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
