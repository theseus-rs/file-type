use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110015790: FileType = FileType {
    file_format: &FileFormat {
        id: 110_015_790,
        source_type: SourceType::Wikidata,
        name: "OrCAD Layout File",
        extensions: &["max"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
