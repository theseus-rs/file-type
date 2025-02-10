use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114093710: FileType = FileType {
    file_format: &FileFormat {
        id: 114_093_710,
        source_type: SourceType::Wikidata,
        name: "Sony SLV File",
        extensions: &["slv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
