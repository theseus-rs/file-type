use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67175538: FileType = FileType {
    file_format: &FileFormat {
        id: 67_175_538,
        source_type: SourceType::Wikidata,
        name: "Nero Digital file",
        extensions: &["nd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
