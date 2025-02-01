use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28756571: FileFormat = FileFormat {
    id: 28_756_571,
    puid: "wikidata/28756571",
    name: "Forth script",
    extensions: &["fth"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
