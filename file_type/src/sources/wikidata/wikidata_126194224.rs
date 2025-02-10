use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126194224: FileType = FileType {
    file_format: &FileFormat {
        id: 126_194_224,
        source_type: SourceType::Wikidata,
        name: "Safetensors",
        extensions: &["safetensors"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
