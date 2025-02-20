use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61990483: FileType = FileType {
    file_format: &FileFormat {
        id: 61_990_483,
        source_type: SourceType::Wikidata,
        name: "Log ASCII Standard Format, version 2",
        extensions: &["las"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
