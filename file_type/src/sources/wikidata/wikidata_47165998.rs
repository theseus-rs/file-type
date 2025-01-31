use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47165998: FileFormat = FileFormat {
    id: 47_165_998,
    puid: "wikidata/47165998",
    name: "ClarisWorks file format version 1",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
