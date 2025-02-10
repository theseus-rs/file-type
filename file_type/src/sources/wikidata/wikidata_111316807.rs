use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111316807: FileType = FileType {
    file_format: &FileFormat {
        id: 111_316_807,
        source_type: SourceType::Wikidata,
        name: "Kurzweil K2500 file",
        extensions: &["k25"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
