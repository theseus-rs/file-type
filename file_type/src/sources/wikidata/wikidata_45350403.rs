use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_45350403: FileType = FileType {
    file_format: &FileFormat {
        id: 45_350_403,
        source_type: SourceType::Wikidata,
        name: "Lotus 1-2-3 Spreadsheet Formatting File, version 2.3-2.4",
        extensions: &["fm1", "fmt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
