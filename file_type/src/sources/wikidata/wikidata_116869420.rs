use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116869420: FileFormat = FileFormat {
    id: 116_869_420,
    puid: "wikidata/116869420",
    name: "Broderbund Print Meta Object",
    extensions: &["pmo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
