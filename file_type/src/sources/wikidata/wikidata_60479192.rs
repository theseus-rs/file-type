use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60479192: FileType = FileType {
    file_format: &FileFormat {
        id: 60_479_192,
        source_type: SourceType::Wikidata,
        name: "Quattro Pro Spreadsheet for Windows",
        extensions: &["wb1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
