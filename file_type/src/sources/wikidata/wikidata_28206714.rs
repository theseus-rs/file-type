use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206714: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_714,
        source_type: SourceType::Wikidata,
        name: "Portable Anymap",
        extensions: &["pnm"],
        media_types: &["image/x-portable-anymap"],
        signatures: &[],
        related_formats: &[],
    },
};
