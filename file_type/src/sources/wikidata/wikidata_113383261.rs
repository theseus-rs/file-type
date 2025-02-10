use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113383261: FileType = FileType {
    file_format: &FileFormat {
        id: 113_383_261,
        source_type: SourceType::Wikidata,
        name: "Roxio Audio Project File 11",
        extensions: &["rox"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
