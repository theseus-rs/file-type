use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61774372: FileType = FileType {
    file_format: &FileFormat {
        id: 61_774_372,
        source_type: SourceType::Wikidata,
        name: "WavPack Binary",
        extensions: &["wv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
