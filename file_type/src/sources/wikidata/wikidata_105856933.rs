use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856933: FileFormat = FileFormat {
    id: 105_856_933,
    puid: "wikidata/105856933",
    name: "NETGEN Constructive Solid Geometry format (with rem)",
    extensions: &["geo"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
