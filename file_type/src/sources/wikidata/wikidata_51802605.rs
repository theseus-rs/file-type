use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51802605: FileType = FileType {
    file_format: &FileFormat {
        id: 51_802_605,
        source_type: SourceType::Wikidata,
        name: "OS/2 Change Control File",
        extensions: &["cin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
