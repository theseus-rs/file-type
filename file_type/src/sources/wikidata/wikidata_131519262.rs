use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131519262: FileType = FileType {
    file_format: &FileFormat {
        id: 131_519_262,
        source_type: SourceType::Wikidata,
        name: "Stimulate Signal Parameters",
        extensions: &["spr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
