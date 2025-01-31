use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25099931: FileFormat = FileFormat {
    id: 25_099_931,
    puid: "wikidata/25099931",
    name: "Scratch Project SB2",
    extensions: &["sb2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
