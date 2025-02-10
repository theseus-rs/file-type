use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1260547: FileType = FileType {
    file_format: &FileFormat {
        id: 1_260_547,
        source_type: SourceType::Wikidata,
        name: "LyRiCs",
        extensions: &["lrc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
