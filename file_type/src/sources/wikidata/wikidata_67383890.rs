use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67383890: FileFormat = FileFormat {
    id: 67_383_890,
    puid: "wikidata/67383890",
    name: "Source Engine Compiled AI Nodegraph",
    extensions: &["ain"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
