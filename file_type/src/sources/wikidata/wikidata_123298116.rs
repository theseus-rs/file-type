use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123298116: FileFormat = FileFormat {
    id: 123_298_116,
    puid: "wikidata/123298116",
    name: "To Do Archive",
    extensions: &["tda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
