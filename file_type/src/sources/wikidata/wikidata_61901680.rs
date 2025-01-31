use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61901680: FileFormat = FileFormat {
    id: 61_901_680,
    puid: "wikidata/61901680",
    name: "EndNote Connection File",
    extensions: &["enz"],
    media_types: &["application/x-endnote-connection"],
    internal_signatures: &[],
    related_formats: &[],
};
