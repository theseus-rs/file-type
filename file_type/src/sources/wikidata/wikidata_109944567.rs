use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109944567: FileFormat = FileFormat {
    id: 109_944_567,
    puid: "wikidata/109944567",
    name: "Generic CADD file format",
    extensions: &["gcd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
