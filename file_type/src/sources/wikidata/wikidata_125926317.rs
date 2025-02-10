use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125926317: FileType = FileType {
    file_format: &FileFormat {
        id: 125_926_317,
        source_type: SourceType::Wikidata,
        name: "SolidWorks Library Feature Part",
        extensions: &["sldlfp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
