use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61774420: FileType = FileType {
    file_format: &FileFormat {
        id: 61_774_420,
        source_type: SourceType::Wikidata,
        name: "WavPack Correction File, version 4",
        extensions: &["wvc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
