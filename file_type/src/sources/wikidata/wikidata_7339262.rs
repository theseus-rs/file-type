use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7339262: FileFormat = FileFormat {
    id: 7_339_262,
    puid: "wikidata/7339262",
    name: "RoadXML",
    extensions: &["rnd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
