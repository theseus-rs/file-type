use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1485172: FileFormat = FileFormat {
    id: 1_485_172,
    puid: "wikidata/1485172",
    name: "GENealogical inDEX",
    extensions: &["gendex.txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
