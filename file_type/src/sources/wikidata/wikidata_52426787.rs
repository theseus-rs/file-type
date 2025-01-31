use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52426787: FileFormat = FileFormat {
    id: 52_426_787,
    puid: "wikidata/52426787",
    name: "XYWrite Document, version III+",
    extensions: &["xyp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
