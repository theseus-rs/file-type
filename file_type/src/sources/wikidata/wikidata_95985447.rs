use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_95985447: FileType = FileType {
    file_format: &FileFormat {
        id: 95_985_447,
        source_type: SourceType::Wikidata,
        name: "Affymetrix GIN file format",
        extensions: &["gin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
