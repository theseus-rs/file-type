use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136431699: FileType = FileType {
    file_format: &FileFormat {
        id: 136_431_699,
        source_type: SourceType::Wikidata,
        name: "Immersive Audio Model Format",
        extensions: &["iamf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
