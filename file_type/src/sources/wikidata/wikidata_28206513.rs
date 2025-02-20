use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206513: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_513,
        source_type: SourceType::Wikidata,
        name: "LSS16",
        extensions: &["16", "lss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
