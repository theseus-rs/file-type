use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123298709: FileFormat = FileFormat {
    id: 123_298_709,
    puid: "wikidata/123298709",
    name: "Retrospect RBC File",
    extensions: &["rbc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
