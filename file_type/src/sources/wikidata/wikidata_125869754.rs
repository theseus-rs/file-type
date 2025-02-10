use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125869754: FileType = FileType {
    file_format: &FileFormat {
        id: 125_869_754,
        source_type: SourceType::Wikidata,
        name: "Pro Tools Session File 5-9",
        extensions: &["ptf", "pts"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
