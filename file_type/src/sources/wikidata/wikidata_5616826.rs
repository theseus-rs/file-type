use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5616826: FileType = FileType {
    file_format: &FileFormat {
        id: 5_616_826,
        source_type: SourceType::Wikidata,
        name: "ChordPro",
        extensions: &["cho", "chopro", "crd", "pro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
