use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117287222: FileType = FileType {
    file_format: &FileFormat {
        id: 117_287_222,
        source_type: SourceType::Wikidata,
        name: "SigmaPlot DOS Worksheet",
        extensions: &["spg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
