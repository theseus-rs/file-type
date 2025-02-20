use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28731055: FileType = FileType {
    file_format: &FileFormat {
        id: 28_731_055,
        source_type: SourceType::Wikidata,
        name: "Ability Spreadsheet Template",
        extensions: &["ast"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
