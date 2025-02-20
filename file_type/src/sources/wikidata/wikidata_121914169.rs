use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121914169: FileType = FileType {
    file_format: &FileFormat {
        id: 121_914_169,
        source_type: SourceType::Wikidata,
        name: "Memory Stick Voice File /Digital Voice File LPEC Codec",
        extensions: &["dvf", "msv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
