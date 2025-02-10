use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205381: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_381,
        source_type: SourceType::Wikidata,
        name: "Lytro",
        extensions: &["lfp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
