use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130240299: FileType = FileType {
    file_format: &FileFormat {
        id: 130_240_299,
        source_type: SourceType::Wikidata,
        name: "Literate Agda source code file",
        extensions: &["lagda"],
        media_types: &["text/x-literate-agda"],
        signatures: &[],
        related_formats: &[],
    },
};
