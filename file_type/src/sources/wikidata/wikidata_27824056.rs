use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27824056: FileType = FileType {
    file_format: &FileFormat {
        id: 27_824_056,
        source_type: SourceType::Wikidata,
        name: "ar, Coherent variant",
        extensions: &["a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
