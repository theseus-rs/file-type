use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206053: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_053,
        source_type: SourceType::Wikidata,
        name: "ERDAS LAN",
        extensions: &["lan"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
