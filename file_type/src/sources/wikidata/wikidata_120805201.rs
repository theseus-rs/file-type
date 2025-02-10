use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120805201: FileType = FileType {
    file_format: &FileFormat {
        id: 120_805_201,
        source_type: SourceType::Wikidata,
        name: "OCP Art Studio Screen File",
        extensions: &["SCR"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
