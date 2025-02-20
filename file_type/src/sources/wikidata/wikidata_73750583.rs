use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_73750583: FileType = FileType {
    file_format: &FileFormat {
        id: 73_750_583,
        source_type: SourceType::Wikidata,
        name: "Intuit QuickBooks for Windows",
        extensions: &["qbw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
