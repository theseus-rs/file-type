use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113083700: FileType = FileType {
    file_format: &FileFormat {
        id: 113_083_700,
        source_type: SourceType::Wikidata,
        name: "Okino Transfer File Format",
        extensions: &["bdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
