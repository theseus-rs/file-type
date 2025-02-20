use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113162157: FileType = FileType {
    file_format: &FileFormat {
        id: 113_162_157,
        source_type: SourceType::Wikidata,
        name: "MyAdvancedLabelDesigner File",
        extensions: &["mlb", "mld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
