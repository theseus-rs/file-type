use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975794: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_794,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version R11/R12",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
