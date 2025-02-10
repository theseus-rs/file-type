use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_64763203: FileType = FileType {
    file_format: &FileFormat {
        id: 64_763_203,
        source_type: SourceType::Wikidata,
        name: "MapPoint template file format",
        extensions: &["ptt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
