use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
