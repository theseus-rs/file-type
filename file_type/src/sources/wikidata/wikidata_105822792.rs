use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105822792: FileType = FileType {
    file_format: &FileFormat {
        id: 105_822_792,
        source_type: SourceType::Wikidata,
        name: "AMDIS Target Compounds Library",
        extensions: &["MSL"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
