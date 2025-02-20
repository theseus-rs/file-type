use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61990487: FileType = FileType {
    file_format: &FileFormat {
        id: 61_990_487,
        source_type: SourceType::Wikidata,
        name: "Log ASCII Standard Format, version 3",
        extensions: &["las"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
