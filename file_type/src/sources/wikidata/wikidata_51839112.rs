use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51839112: FileFormat = FileFormat {
    id: 51_839_112,
    puid: "wikidata/51839112",
    name: "AutoCAD Film Roll",
    extensions: &["flm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
