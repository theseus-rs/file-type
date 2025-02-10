use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27826464: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_464,
        source_type: SourceType::Wikidata,
        name: "Cascading Style Sheets Level 1",
        extensions: &["css"],
        media_types: &["text/css"],
        signatures: &[],
        related_formats: &[],
    },
};
