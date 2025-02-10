use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3841253: FileType = FileType {
    file_format: &FileFormat {
        id: 3_841_253,
        source_type: SourceType::Wikidata,
        name: "MDL Molfile",
        extensions: &["mol"],
        media_types: &["chemical/x-mdl-molfile"],
        signatures: &[],
        related_formats: &[],
    },
};
