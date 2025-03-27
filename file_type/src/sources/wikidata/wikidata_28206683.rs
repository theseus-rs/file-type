use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206683: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_683,
        source_type: SourceType::Wikidata,
        name: "Portable Graymap, text variant",
        extensions: &["pgm", "pnm"],
        media_types: &["image/x-portable-anymap", "image/x-portable-graymap"],
        signatures: &[],
        related_formats: &[],
    },
};
