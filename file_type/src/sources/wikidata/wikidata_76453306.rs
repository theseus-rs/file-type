use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_76453306: FileType = FileType {
    file_format: &FileFormat {
        id: 76_453_306,
        source_type: SourceType::Wikidata,
        name: "MagicPoint presentation format",
        extensions: &["mgp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
