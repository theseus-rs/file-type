use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29141370: FileType = FileType {
    file_format: &FileFormat {
        id: 29_141_370,
        source_type: SourceType::Wikidata,
        name: "cpio, old binary variant, little-endian encoding",
        extensions: &["cpio"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
