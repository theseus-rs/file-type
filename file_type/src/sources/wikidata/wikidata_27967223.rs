use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967223: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_223,
        source_type: SourceType::Wikidata,
        name: "StarTrekker/Star Tracker module",
        extensions: &["mod", "nt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
