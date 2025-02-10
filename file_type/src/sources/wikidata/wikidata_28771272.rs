use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28771272: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_272,
        source_type: SourceType::Wikidata,
        name: "MVG",
        extensions: &["mvg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
