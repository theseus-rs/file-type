use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122946779: FileType = FileType {
    file_format: &FileFormat {
        id: 122_946_779,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version 2004-2005",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
