use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134692295: FileType = FileType {
    file_format: &FileFormat {
        id: 134_692_295,
        source_type: SourceType::Wikidata,
        name: "NooJ productive morphological grammar file",
        extensions: &["nom"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
