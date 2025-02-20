use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600496: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_496,
        source_type: SourceType::Wikidata,
        name: "diff",
        extensions: &["diff", "patch"],
        media_types: &["text/x-patch"],
        signatures: &[],
        related_formats: &[],
    },
};
