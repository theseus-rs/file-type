use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118672139: FileType = FileType {
    file_format: &FileFormat {
        id: 118_672_139,
        source_type: SourceType::Wikidata,
        name: "Manga Studio 1.0 Document",
        extensions: &["mpf", "msf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
