use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27826469: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_469,
        source_type: SourceType::Wikidata,
        name: "Cascading Style Sheets Level 3",
        extensions: &["css"],
        media_types: &["text/css"],
        signatures: &[],
        related_formats: &[],
    },
};
