use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
