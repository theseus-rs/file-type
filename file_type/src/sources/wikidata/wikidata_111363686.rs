use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111363686: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_686,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif XS 'voices' format",
        extensions: &["x0v"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
