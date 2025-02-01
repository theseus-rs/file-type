use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131418899: FileFormat = FileFormat {
    id: 131_418_899,
    puid: "wikidata/131418899",
    name: "Web IDL file format",
    extensions: &["webidl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
