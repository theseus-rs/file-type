use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_124158014: FileType = FileType {
    file_format: &FileFormat {
        id: 124_158_014,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version 2007-2008-2009",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
