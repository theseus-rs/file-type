use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
