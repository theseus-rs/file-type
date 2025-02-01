use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206378: FileFormat = FileFormat {
    id: 28_206_378,
    puid: "wikidata/28206378",
    name: "IPI",
    extensions: &["ipi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
