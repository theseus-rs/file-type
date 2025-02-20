use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60887256: FileType = FileType {
    file_format: &FileFormat {
        id: 60_887_256,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version 1.1",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
