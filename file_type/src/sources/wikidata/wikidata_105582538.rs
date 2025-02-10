use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_105582538: FileType = FileType {
    file_format: &FileFormat {
        id: 105_582_538,
        source_type: SourceType::Wikidata,
        name: "JavaScript modules",
        extensions: &["mjs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
