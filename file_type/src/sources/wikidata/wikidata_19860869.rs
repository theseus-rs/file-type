use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_19860869: FileType = FileType {
    file_format: &FileFormat {
        id: 19_860_869,
        source_type: SourceType::Wikidata,
        name: "Itinerary file",
        extensions: &["itn"],
        media_types: &["application/itn"],
        signatures: &[],
        related_formats: &[],
    },
};
