use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206272: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_272,
        source_type: SourceType::Wikidata,
        name: "HTC splashscreen",
        extensions: &["img", "nb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
