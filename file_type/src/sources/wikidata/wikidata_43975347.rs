use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_43975347: FileType = FileType {
    file_format: &FileFormat {
        id: 43_975_347,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (Binary), version R14",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
