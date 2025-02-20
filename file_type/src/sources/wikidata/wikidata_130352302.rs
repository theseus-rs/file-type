use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130352302: FileType = FileType {
    file_format: &FileFormat {
        id: 130_352_302,
        source_type: SourceType::Wikidata,
        name: "Monte file",
        extensions: &["mt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
