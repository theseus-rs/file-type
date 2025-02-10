use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111263007: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_007,
        source_type: SourceType::Wikidata,
        name: "Velvet Studio sample",
        extensions: &["ase"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
