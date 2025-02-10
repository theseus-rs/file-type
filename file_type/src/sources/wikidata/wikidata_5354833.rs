use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5354833: FileType = FileType {
    file_format: &FileFormat {
        id: 5_354_833,
        source_type: SourceType::Wikidata,
        name: "Qualcomm code-excited linear prediction",
        extensions: &["qcp"],
        media_types: &["audio/qcelp"],
        signatures: &[],
        related_formats: &[],
    },
};
