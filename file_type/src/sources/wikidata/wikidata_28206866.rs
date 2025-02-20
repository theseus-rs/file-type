use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206866: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_866,
        source_type: SourceType::Wikidata,
        name: "PCPaint clipping format",
        extensions: &["clp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
