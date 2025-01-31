use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000691: FileFormat = FileFormat {
    id: 29_000_691,
    puid: "wikidata/29000691",
    name: "RayShade Scene Format",
    extensions: &["ray"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
