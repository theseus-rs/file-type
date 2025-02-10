use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_96052469: FileType = FileType {
    file_format: &FileFormat {
        id: 96_052_469,
        source_type: SourceType::Wikidata,
        name: "Mathematica Graphics Format",
        extensions: &["mgf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
