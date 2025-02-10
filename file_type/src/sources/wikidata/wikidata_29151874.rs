use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29151874: FileType = FileType {
    file_format: &FileFormat {
        id: 29_151_874,
        source_type: SourceType::Wikidata,
        name: "QRT Ray Tracer scene description",
        extensions: &["qrt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
