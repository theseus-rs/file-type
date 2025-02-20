use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29905206: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_206,
        source_type: SourceType::Wikidata,
        name: "Self-Dissolving Archive",
        extensions: &["sda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
