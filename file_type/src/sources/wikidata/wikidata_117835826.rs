use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117835826: FileType = FileType {
    file_format: &FileFormat {
        id: 117_835_826,
        source_type: SourceType::Wikidata,
        name: "Fujitsu DexNET file",
        extensions: &["dxn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
