use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206579: FileFormat = FileFormat {
    id: 28_206_579,
    puid: "wikidata/28206579",
    name: "MetaMorph Stack",
    extensions: &["stk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
