use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_370979: FileFormat = FileFormat {
    id: 370_979,
    puid: "wikidata/370979",
    name: "Amigaguide",
    extensions: &["guide"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
