use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110984245: FileType = FileType {
    file_format: &FileFormat {
        id: 110_984_245,
        source_type: SourceType::Wikidata,
        name: "Video Paint File",
        extensions: &["uvp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
