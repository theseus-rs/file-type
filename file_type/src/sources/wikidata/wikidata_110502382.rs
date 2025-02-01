use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110502382: FileFormat = FileFormat {
    id: 110_502_382,
    puid: "wikidata/110502382",
    name: "ISDOC Information System Document (Generic)",
    extensions: &["isdoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
