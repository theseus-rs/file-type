use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51844052: FileType = FileType {
    file_format: &FileFormat {
        id: 51_844_052,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visual Modeller Petal file (ASCII)",
        extensions: &["ptl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
