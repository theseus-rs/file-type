use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_84842870: FileType = FileType {
    file_format: &FileFormat {
        id: 84_842_870,
        source_type: SourceType::Wikidata,
        name: "GL Transmission Format, version 1 (text)",
        extensions: &["gltf"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
