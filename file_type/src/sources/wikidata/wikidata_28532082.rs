use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28532082: FileType = FileType {
    file_format: &FileFormat {
        id: 28_532_082,
        source_type: SourceType::Wikidata,
        name: "CAChe MolStruct",
        extensions: &["cac", "cache"],
        media_types: &["chemical/x-cache"],
        signatures: &[],
        related_formats: &[],
    },
};
