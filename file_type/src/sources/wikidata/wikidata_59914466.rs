use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59914466: FileFormat = FileFormat {
    id: 59_914_466,
    puid: "wikidata/59914466",
    name: "Deluxe Paint bitmap",
    extensions: &["lbm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
