use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120000309: FileType = FileType {
    file_format: &FileFormat {
        id: 120_000_309,
        source_type: SourceType::Wikidata,
        name: "ASAP WordPower Presentation",
        extensions: &["asp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
