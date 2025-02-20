use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
