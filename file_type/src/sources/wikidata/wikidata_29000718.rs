use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000718: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_718,
        source_type: SourceType::Wikidata,
        name: "Unreal model aniv file",
        extensions: &["3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
