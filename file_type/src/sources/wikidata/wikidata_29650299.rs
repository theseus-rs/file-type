use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650299: FileFormat = FileFormat {
    id: 29_650_299,
    puid: "wikidata/29650299",
    name: "PUZ",
    extensions: &["puz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
