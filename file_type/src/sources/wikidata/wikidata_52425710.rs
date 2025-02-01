use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52425710: FileFormat = FileFormat {
    id: 52_425_710,
    puid: "wikidata/52425710",
    name: "VisiCalc Database",
    extensions: &["dif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
