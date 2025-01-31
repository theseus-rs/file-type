use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131278493: FileFormat = FileFormat {
    id: 131_278_493,
    puid: "wikidata/131278493",
    name: "Test Anything Protocol output file",
    extensions: &["tap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
