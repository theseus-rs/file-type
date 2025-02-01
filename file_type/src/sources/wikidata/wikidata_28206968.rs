use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206968: FileFormat = FileFormat {
    id: 28_206_968,
    puid: "wikidata/28206968",
    name: "Photoshop brush",
    extensions: &["abr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
