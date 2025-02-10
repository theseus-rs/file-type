use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121913880: FileType = FileType {
    file_format: &FileFormat {
        id: 121_913_880,
        source_type: SourceType::Wikidata,
        name: "Memory Stick Voice File ADPCM Codec",
        extensions: &["msv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
