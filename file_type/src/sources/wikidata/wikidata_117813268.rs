use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117813268: FileFormat = FileFormat {
    id: 117_813_268,
    puid: "wikidata/117813268",
    name: "Meter Data",
    extensions: &["dta"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
