use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118877820: FileType = FileType {
    file_format: &FileFormat {
        id: 118_877_820,
        source_type: SourceType::Wikidata,
        name: "Open Scripting Architecture binary script",
        extensions: &["scpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
