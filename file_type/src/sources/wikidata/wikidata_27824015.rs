use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27824015: FileType = FileType {
    file_format: &FileFormat {
        id: 27_824_015,
        source_type: SourceType::Wikidata,
        name: "ar, BSD variant",
        extensions: &["a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
