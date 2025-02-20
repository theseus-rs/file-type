use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975796: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_796,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version R10",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
