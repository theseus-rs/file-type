use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130224300: FileType = FileType {
    file_format: &FileFormat {
        id: 130_224_300,
        source_type: SourceType::Wikidata,
        name: "Lean 4 file format",
        extensions: &["lean"],
        media_types: &["text/x-lean4"],
        signatures: &[],
        related_formats: &[],
    },
};
