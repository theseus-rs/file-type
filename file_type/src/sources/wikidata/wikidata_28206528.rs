use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206528: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_528,
        source_type: SourceType::Wikidata,
        name: "MacPaint",
        extensions: &["mac", "pic", "pntg"],
        media_types: &["image/x-macpaint"],
        signatures: &[],
        related_formats: &[],
    },
};
