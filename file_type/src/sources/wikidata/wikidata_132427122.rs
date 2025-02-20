use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132427122: FileType = FileType {
    file_format: &FileFormat {
        id: 132_427_122,
        source_type: SourceType::Wikidata,
        name: "CBOR",
        extensions: &["cbor"],
        media_types: &["application/cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
