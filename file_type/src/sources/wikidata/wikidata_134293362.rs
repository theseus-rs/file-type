use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134293362: FileType = FileType {
    file_format: &FileFormat {
        id: 134_293_362,
        source_type: SourceType::Wikidata,
        name: "Clipper memory allocation file",
        extensions: &["map"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
