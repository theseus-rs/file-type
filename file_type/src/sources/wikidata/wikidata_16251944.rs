use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_16251944: FileFormat = FileFormat {
    id: 16_251_944,
    puid: "wikidata/16251944",
    name: "TransXChange",
    extensions: &["txc", "xml"],
    media_types: &["application/xml", "application/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
