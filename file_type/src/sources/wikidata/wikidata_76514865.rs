use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_76514865: FileType = FileType {
    file_format: &FileFormat {
        id: 76_514_865,
        source_type: SourceType::Wikidata,
        name: "WinDev Report",
        extensions: &["wde"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
