use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125926204: FileType = FileType {
    file_format: &FileFormat {
        id: 125_926_204,
        source_type: SourceType::Wikidata,
        name: "Solidworks Slide File",
        extensions: &["sld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
