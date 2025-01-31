use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75540713: FileFormat = FileFormat {
    id: 75_540_713,
    puid: "wikidata/75540713",
    name: "Ulead PhotoImpact Object",
    extensions: &["ufo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
