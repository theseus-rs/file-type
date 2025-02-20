use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207427: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_427,
        source_type: SourceType::Wikidata,
        name: "Verity Image Format",
        extensions: &["vif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
