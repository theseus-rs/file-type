use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118218029: FileType = FileType {
    file_format: &FileFormat {
        id: 118_218_029,
        source_type: SourceType::Wikidata,
        name: "FotoFinish Layout",
        extensions: &["fdd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
