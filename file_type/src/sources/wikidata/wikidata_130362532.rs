use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130362532: FileFormat = FileFormat {
    id: 130_362_532,
    puid: "wikidata/130362532",
    name: "MuPAD file format",
    extensions: &["mu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
