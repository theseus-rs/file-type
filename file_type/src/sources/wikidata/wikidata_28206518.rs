use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206518: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_518,
        source_type: SourceType::Wikidata,
        name: "Lucasfilm picture",
        extensions: &["lff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
