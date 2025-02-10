use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28344485: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_485,
        source_type: SourceType::Wikidata,
        name: "HKI",
        extensions: &["hki"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
