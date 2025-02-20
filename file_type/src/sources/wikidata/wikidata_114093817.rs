use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114093817: FileType = FileType {
    file_format: &FileFormat {
        id: 114_093_817,
        source_type: SourceType::Wikidata,
        name: "Media Hash List",
        extensions: &["mhl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
