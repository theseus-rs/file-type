use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61774392: FileType = FileType {
    file_format: &FileFormat {
        id: 61_774_392,
        source_type: SourceType::Wikidata,
        name: "WavPack Correction File",
        extensions: &["wvc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
