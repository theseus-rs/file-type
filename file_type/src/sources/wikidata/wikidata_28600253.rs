use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600253: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_253,
        source_type: SourceType::Wikidata,
        name: ".art",
        extensions: &["art"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
