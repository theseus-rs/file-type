use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111521910: FileFormat = FileFormat {
    id: 111_521_910,
    puid: "wikidata/111521910",
    name: "Packed-Ice True Colour Picture",
    extensions: &["trp", "tru"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
