use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27824041: FileType = FileType {
    file_format: &FileFormat {
        id: 27_824_041,
        source_type: SourceType::Wikidata,
        name: "ar, Seventh Edition Unix variant",
        extensions: &["a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
