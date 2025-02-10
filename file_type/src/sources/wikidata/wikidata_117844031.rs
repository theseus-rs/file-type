use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117844031: FileType = FileType {
    file_format: &FileFormat {
        id: 117_844_031,
        source_type: SourceType::Wikidata,
        name: "Inset Systems IGF format",
        extensions: &["igf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
