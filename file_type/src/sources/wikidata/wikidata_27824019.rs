use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27824019: FileType = FileType {
    file_format: &FileFormat {
        id: 27_824_019,
        source_type: SourceType::Wikidata,
        name: "ar, System V variant",
        extensions: &["a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
