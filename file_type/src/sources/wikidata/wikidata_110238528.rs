use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110238528: FileType = FileType {
    file_format: &FileFormat {
        id: 110_238_528,
        source_type: SourceType::Wikidata,
        name: "Screenwriter 2000 Document",
        extensions: &["stw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
