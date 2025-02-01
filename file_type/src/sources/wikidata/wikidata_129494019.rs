use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129494019: FileFormat = FileFormat {
    id: 129_494_019,
    puid: "wikidata/129494019",
    name: "GSQL query file",
    extensions: &["gsql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
