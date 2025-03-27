use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29141576: FileType = FileType {
    file_format: &FileFormat {
        id: 29_141_576,
        source_type: SourceType::Wikidata,
        name: "cpio, new CRC variant",
        extensions: &["cpio"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
