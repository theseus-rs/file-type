use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138644007: FileType = FileType {
    file_format: &FileFormat {
        id: 138_644_007,
        source_type: SourceType::Wikidata,
        name: "Transportable Layout Cell",
        extensions: &["tlc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
