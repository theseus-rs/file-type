use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28344215: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_215,
        source_type: SourceType::Wikidata,
        name: "Application Developer Documentation Index",
        extensions: &["axc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
