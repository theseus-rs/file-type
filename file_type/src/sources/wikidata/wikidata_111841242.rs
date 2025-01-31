use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111841242: FileFormat = FileFormat {
    id: 111_841_242,
    puid: "wikidata/111841242",
    name: "NDJSON",
    extensions: &["ndjson"],
    media_types: &["application/x-ndjson"],
    internal_signatures: &[],
    related_formats: &[],
};
