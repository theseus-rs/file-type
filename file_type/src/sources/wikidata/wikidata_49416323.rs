use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49416323: FileFormat = FileFormat {
    id: 49_416_323,
    source_type: SourceType::Wikidata,
    name: "CATIA Model (Part Description), version 5",
    extensions: &["catpart"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
