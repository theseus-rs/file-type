use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109334805: FileType = FileType {
    file_format: &FileFormat {
        id: 109_334_805,
        source_type: SourceType::Wikidata,
        name: "Advanced Comic Book Format",
        extensions: &["acbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
