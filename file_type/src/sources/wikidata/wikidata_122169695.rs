use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122169695: FileType = FileType {
    file_format: &FileFormat {
        id: 122_169_695,
        source_type: SourceType::Wikidata,
        name: "Key Cache File",
        extensions: &["ekc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
