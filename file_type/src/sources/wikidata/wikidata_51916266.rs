use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51916266: FileType = FileType {
    file_format: &FileFormat {
        id: 51_916_266,
        source_type: SourceType::Wikidata,
        name: "CCITT G.711 Audio",
        extensions: &["ulaw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
