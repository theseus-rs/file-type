use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_572649: FileType = FileType {
    file_format: &FileFormat {
        id: 572_649,
        source_type: SourceType::Wikidata,
        name: "Intel HEX",
        extensions: &["hex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
