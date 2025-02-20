use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113383223: FileType = FileType {
    file_format: &FileFormat {
        id: 113_383_223,
        source_type: SourceType::Wikidata,
        name: "Roxio Data Project File 11",
        extensions: &["rox"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
