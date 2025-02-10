use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28777687: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_687,
        source_type: SourceType::Wikidata,
        name: "Mono",
        extensions: &["mono"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
