use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_10465505: FileType = FileType {
    file_format: &FileFormat {
        id: 10_465_505,
        source_type: SourceType::Wikidata,
        name: "DTS-HD",
        extensions: &["dtshd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
