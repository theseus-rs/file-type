use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2304584: FileType = FileType {
    file_format: &FileFormat {
        id: 2_304_584,
        source_type: SourceType::Wikidata,
        name: "Windows Imaging Format",
        extensions: &["rwm", "swm", "wim"],
        media_types: &["application/x-ms-wim"],
        signatures: &[],
        related_formats: &[],
    },
};
