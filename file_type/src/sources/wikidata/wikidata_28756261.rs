use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28756261: FileType = FileType {
    file_format: &FileFormat {
        id: 28_756_261,
        source_type: SourceType::Wikidata,
        name: "FIG",
        extensions: &["fig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
