use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130713731: FileType = FileType {
    file_format: &FileFormat {
        id: 130_713_731,
        source_type: SourceType::Wikidata,
        name: "RSL file format",
        extensions: &["rsl"],
        media_types: &["text/rsl"],
        signatures: &[],
        related_formats: &[],
    },
};
