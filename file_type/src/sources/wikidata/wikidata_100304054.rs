use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100304054: FileType = FileType {
    file_format: &FileFormat {
        id: 100_304_054,
        source_type: SourceType::Wikidata,
        name: "Flow Charting Graphic Flowcharting Image",
        extensions: &["gfi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
