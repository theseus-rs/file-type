use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49416323: FileFormat = FileFormat {
    id: 49_416_323,
    puid: "wikidata/49416323",
    name: "CATIA Model (Part Description), version 5",
    extensions: &["catpart"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
