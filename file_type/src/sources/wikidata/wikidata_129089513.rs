use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129089513: FileType = FileType {
    file_format: &FileFormat {
        id: 129_089_513,
        source_type: SourceType::Wikidata,
        name: "Embedded Ragel file",
        extensions: &["rl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
