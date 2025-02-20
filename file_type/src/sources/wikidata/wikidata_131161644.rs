use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
