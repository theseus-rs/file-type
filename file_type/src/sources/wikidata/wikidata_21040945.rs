use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21040945: FileFormat = FileFormat {
    id: 21_040_945,
    puid: "wikidata/21040945",
    name: "Digitrakker format",
    extensions: &["mdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
