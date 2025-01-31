use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_474112: FileFormat = FileFormat {
    id: 474_112,
    puid: "wikidata/474112",
    name: "JHTML",
    extensions: &["jhtml"],
    media_types: &["java-internal/java-html"],
    internal_signatures: &[],
    related_formats: &[],
};
