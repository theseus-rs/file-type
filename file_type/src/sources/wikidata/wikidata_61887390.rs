use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61887390: FileFormat = FileFormat {
    id: 61_887_390,
    puid: "wikidata/61887390",
    name: "EndNote Library format 1-9",
    extensions: &["enl"],
    media_types: &["application/x-endnote-library"],
    internal_signatures: &[],
    related_formats: &[],
};
