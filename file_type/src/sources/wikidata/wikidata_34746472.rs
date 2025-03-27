use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34746472: FileType = FileType {
    file_format: &FileFormat {
        id: 34_746_472,
        source_type: SourceType::Wikidata,
        name: "STATISTICA Spreadsheet File",
        extensions: &["css", "sta"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
