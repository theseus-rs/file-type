use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34745668: FileType = FileType {
    file_format: &FileFormat {
        id: 34_745_668,
        source_type: SourceType::Wikidata,
        name: "Squeeze",
        extensions: &["qqq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
