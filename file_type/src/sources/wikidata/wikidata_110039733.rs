use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110039733: FileType = FileType {
    file_format: &FileFormat {
        id: 110_039_733,
        source_type: SourceType::Wikidata,
        name: "Mar Archive",
        extensions: &["mac", "mar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
