use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61774422: FileType = FileType {
    file_format: &FileFormat {
        id: 61_774_422,
        source_type: SourceType::Wikidata,
        name: "WavPack Correction File, version 5",
        extensions: &["wvc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
