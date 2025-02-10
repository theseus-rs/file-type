use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119257000: FileType = FileType {
    file_format: &FileFormat {
        id: 119_257_000,
        source_type: SourceType::Wikidata,
        name: "PayCycle Import Data",
        extensions: &["pcif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
