use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206561: FileFormat = FileFormat {
    id: 28_206_561,
    puid: "wikidata/28206561",
    name: "Maya IFF",
    extensions: &["iff", "tdi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
