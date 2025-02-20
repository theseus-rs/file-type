use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650337: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_337,
        source_type: SourceType::Wikidata,
        name: "PUPA Font Format, version 2",
        extensions: &["pf2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
