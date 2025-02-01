use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121158405: FileFormat = FileFormat {
    id: 121_158_405,
    puid: "wikidata/121158405",
    name: "Merge file",
    extensions: &["mrg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
