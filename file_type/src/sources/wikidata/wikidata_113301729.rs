use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113301729: FileType = FileType {
    file_format: &FileFormat {
        id: 113_301_729,
        source_type: SourceType::Wikidata,
        name: "Sonic Foundry Audio",
        extensions: &["sfa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
