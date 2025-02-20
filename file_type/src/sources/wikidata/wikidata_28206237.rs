use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206237: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_237,
        source_type: SourceType::Wikidata,
        name: "GROB",
        extensions: &["grb", "gro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
