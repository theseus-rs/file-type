use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111283602: FileType = FileType {
    file_format: &FileFormat {
        id: 111_283_602,
        source_type: SourceType::Wikidata,
        name: "Casio FZ-1 full dump format",
        extensions: &["fzf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
