use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131161644: FileType = FileType {
    file_format: &FileFormat {
        id: 131_161_644,
        source_type: SourceType::Wikidata,
        name: "SRCINFO file format",
        extensions: &["SRCINFO"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
