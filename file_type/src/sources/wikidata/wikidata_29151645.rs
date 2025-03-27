use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29151645: FileType = FileType {
    file_format: &FileFormat {
        id: 29_151_645,
        source_type: SourceType::Wikidata,
        name: "Research Articles in Simplified HTML",
        extensions: &["html"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
