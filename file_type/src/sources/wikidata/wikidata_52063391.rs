use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52063391: FileType = FileType {
    file_format: &FileFormat {
        id: 52_063_391,
        source_type: SourceType::Wikidata,
        name: "SuperCalc Spreadsheet, version 5",
        extensions: &["cal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
