use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28212272: FileFormat = FileFormat {
    id: 28_212_272,
    puid: "wikidata/28212272",
    name: "Noweb",
    extensions: &["nw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
