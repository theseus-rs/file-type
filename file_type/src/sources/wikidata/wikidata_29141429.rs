use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29141429: FileType = FileType {
    file_format: &FileFormat {
        id: 29_141_429,
        source_type: SourceType::Wikidata,
        name: "cpio, old binary variant, big-endian encoding",
        extensions: &["cpio"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
