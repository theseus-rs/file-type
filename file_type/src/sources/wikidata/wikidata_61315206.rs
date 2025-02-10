use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61315206: FileType = FileType {
    file_format: &FileFormat {
        id: 61_315_206,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version 2.5",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
