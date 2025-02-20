use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71001254: FileType = FileType {
    file_format: &FileFormat {
        id: 71_001_254,
        source_type: SourceType::Wikidata,
        name: "Gameboy Sound Format",
        extensions: &["gsf", "gsflib", "minigsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
