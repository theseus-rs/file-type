use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207457: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_457,
        source_type: SourceType::Wikidata,
        name: "Vista Plain File Format",
        extensions: &["vp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
