use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28344449: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_449,
        source_type: SourceType::Wikidata,
        name: "SNSF",
        extensions: &["minisnsf", "snsf", "snsflib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
