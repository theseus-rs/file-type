use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125599381: FileType = FileType {
    file_format: &FileFormat {
        id: 125_599_381,
        source_type: SourceType::Wikidata,
        name: "Kodak Photo-Enhancer File",
        extensions: &["dc2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
