use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111841303: FileFormat = FileFormat {
    id: 111_841_303,
    puid: "wikidata/111841303",
    name: "line-delimited JSON",
    extensions: &["ldjson"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
