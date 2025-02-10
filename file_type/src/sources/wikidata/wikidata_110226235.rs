use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110226235: FileType = FileType {
    file_format: &FileFormat {
        id: 110_226_235,
        source_type: SourceType::Wikidata,
        name: "NeoDesk Icon File, version 3",
        extensions: &["nic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
