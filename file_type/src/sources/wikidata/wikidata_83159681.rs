use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83159681: FileFormat = FileFormat {
    id: 83_159_681,
    puid: "wikidata/83159681",
    name: "RWL",
    extensions: &["rwl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
