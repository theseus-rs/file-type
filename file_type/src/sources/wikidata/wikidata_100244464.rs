use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100244464: FileFormat = FileFormat {
    id: 100_244_464,
    puid: "wikidata/100244464",
    name: "Student Writing Center Letter",
    extensions: &["lt", "ltt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
