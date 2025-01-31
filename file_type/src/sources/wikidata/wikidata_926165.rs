use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_926165: FileFormat = FileFormat {
    id: 926_165,
    puid: "wikidata/926165",
    name: "Geography Markup Language",
    extensions: &["gml"],
    media_types: &["application/gml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
