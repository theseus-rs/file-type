use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2313301: FileType = FileType {
    file_format: &FileFormat {
        id: 2_313_301,
        source_type: SourceType::Wikidata,
        name: "SpreadsheetML",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
