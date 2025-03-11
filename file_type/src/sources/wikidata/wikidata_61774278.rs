use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61774278: FileType = FileType {
    file_format: &FileFormat {
        id: 61_774_278,
        source_type: SourceType::Wikidata,
        name: "WavPack Binary, version 5",
        extensions: &[],
        media_types: &["audio/x-wv"],
        signatures: &[],
        related_formats: &[],
    },
};
