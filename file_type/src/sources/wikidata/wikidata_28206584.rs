use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206584: FileFormat = FileFormat {
    id: 28_206_584,
    puid: "wikidata/28206584",
    name: "MGR bitmap",
    extensions: &["mgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
