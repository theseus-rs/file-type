use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113556941: FileType = FileType {
    file_format: &FileFormat {
        id: 113_556_941,
        source_type: SourceType::Wikidata,
        name: "CDR-Win Image",
        extensions: &["bin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
