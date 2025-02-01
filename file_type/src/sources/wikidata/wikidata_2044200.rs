use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2044200: FileFormat = FileFormat {
    id: 2_044_200,
    puid: "wikidata/2044200",
    name: "PICT",
    extensions: &["pct", "pict"],
    media_types: &["image/x-pict", "image/x-pict"],
    internal_signatures: &[],
    related_formats: &[],
};
