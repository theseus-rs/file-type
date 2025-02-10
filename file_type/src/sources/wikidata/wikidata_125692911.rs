use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125692911: FileType = FileType {
    file_format: &FileFormat {
        id: 125_692_911,
        source_type: SourceType::Wikidata,
        name: "StarImpress 4.0/5.0",
        extensions: &["sdp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
