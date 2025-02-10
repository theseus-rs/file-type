use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100243790: FileType = FileType {
    file_format: &FileFormat {
        id: 100_243_790,
        source_type: SourceType::Wikidata,
        name: "Student Writing Center Report",
        extensions: &["rp", "rpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
