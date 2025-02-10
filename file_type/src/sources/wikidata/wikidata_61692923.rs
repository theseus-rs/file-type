use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61692923: FileType = FileType {
    file_format: &FileFormat {
        id: 61_692_923,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (Binary), version R10",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
