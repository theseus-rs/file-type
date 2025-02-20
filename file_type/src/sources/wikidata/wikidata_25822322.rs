use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_25822322: FileType = FileType {
    file_format: &FileFormat {
        id: 25_822_322,
        source_type: SourceType::Wikidata,
        name: "Protocolbuffer Binary Format",
        extensions: &["osm.pbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
