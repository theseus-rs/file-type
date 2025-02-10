use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112581715: FileType = FileType {
    file_format: &FileFormat {
        id: 112_581_715,
        source_type: SourceType::Wikidata,
        name: "WAN",
        extensions: &["wan"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
