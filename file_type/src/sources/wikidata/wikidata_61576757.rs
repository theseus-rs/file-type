use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61576757: FileType = FileType {
    file_format: &FileFormat {
        id: 61_576_757,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version R14",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
