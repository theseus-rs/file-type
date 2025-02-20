use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62414914: FileType = FileType {
    file_format: &FileFormat {
        id: 62_414_914,
        source_type: SourceType::Wikidata,
        name: "Quattro Pro Spreadsheet, version 9",
        extensions: &["qpw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
