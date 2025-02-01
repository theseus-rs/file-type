use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000664: FileFormat = FileFormat {
    id: 29_000_664,
    puid: "wikidata/29000664",
    name: "Processed Volume",
    extensions: &["pvl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
