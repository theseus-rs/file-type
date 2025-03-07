use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132890817: FileType = FileType {
    file_format: &FileFormat {
        id: 132_890_817,
        source_type: SourceType::Wikidata,
        name: "KryoFlux Stream 3",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
