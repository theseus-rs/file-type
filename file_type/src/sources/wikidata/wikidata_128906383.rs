use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128906383: FileType = FileType {
    file_format: &FileFormat {
        id: 128_906_383,
        source_type: SourceType::Wikidata,
        name: "Dylan session file",
        extensions: &["dylan-console"],
        media_types: &["text/x-dylan-console"],
        signatures: &[],
        related_formats: &[],
    },
};
