use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28455556: FileType = FileType {
    file_format: &FileFormat {
        id: 28_455_556,
        source_type: SourceType::Wikidata,
        name: "CBOR",
        extensions: &["cbor"],
        media_types: &["application/cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
