use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60480274: FileType = FileType {
    file_format: &FileFormat {
        id: 60_480_274,
        source_type: SourceType::Wikidata,
        name: "Quattro Pro Spreadsheet for Windows, version 6",
        extensions: &["wb2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
