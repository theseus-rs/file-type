use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49416639: FileFormat = FileFormat {
    id: 49_416_639,
    source_type: SourceType::Wikidata,
    name: "CATIA Product Description, version 5",
    extensions: &["catproduct"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
