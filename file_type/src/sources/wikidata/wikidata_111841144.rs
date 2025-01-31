use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111841144: FileFormat = FileFormat {
    id: 111_841_144,
    puid: "wikidata/111841144",
    name: "JSON Lines",
    extensions: &["jsonl"],
    media_types: &["application/jsonl"],
    internal_signatures: &[],
    related_formats: &[],
};
