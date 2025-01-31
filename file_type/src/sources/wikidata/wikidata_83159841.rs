use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83159841: FileFormat = FileFormat {
    id: 83_159_841,
    puid: "wikidata/83159841",
    name: "CRN",
    extensions: &["crn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
