use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29141541: FileType = FileType {
    file_format: &FileFormat {
        id: 29_141_541,
        source_type: SourceType::Wikidata,
        name: "cpio, new ASCII variant",
        extensions: &["cpio"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
