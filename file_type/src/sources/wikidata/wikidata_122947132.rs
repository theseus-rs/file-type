use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122947132: FileType = FileType {
    file_format: &FileFormat {
        id: 122_947_132,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version 2010-2012",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
