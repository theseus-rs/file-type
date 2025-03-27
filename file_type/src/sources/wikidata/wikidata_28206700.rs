use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206700: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_700,
        source_type: SourceType::Wikidata,
        name: "Portable Pixmap, text variant",
        extensions: &["pnm", "ppm"],
        media_types: &["image/x-portable-anymap", "image/x-portable-pixmap"],
        signatures: &[],
        related_formats: &[],
    },
};
