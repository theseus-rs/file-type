use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128612807: FileType = FileType {
    file_format: &FileFormat {
        id: 128_612_807,
        source_type: SourceType::Wikidata,
        name: "Abstract Syntax Notation One format",
        extensions: &["asn1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
