use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_73514063: FileType = FileType {
    file_format: &FileFormat {
        id: 73_514_063,
        source_type: SourceType::Wikidata,
        name: "PlayStation Archive",
        extensions: &["psarc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
