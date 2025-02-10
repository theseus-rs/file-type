use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111330701: FileType = FileType {
    file_format: &FileFormat {
        id: 111_330_701,
        source_type: SourceType::Wikidata,
        name: "MadTracker 2 instruments",
        extensions: &["mti"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
