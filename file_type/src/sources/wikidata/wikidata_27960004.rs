use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27960004: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_004,
        source_type: SourceType::Wikidata,
        name: "Real Lossless Codec",
        extensions: &["rmvb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
