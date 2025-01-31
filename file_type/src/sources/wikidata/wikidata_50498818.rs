use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50498818: FileFormat = FileFormat {
    id: 50_498_818,
    puid: "wikidata/50498818",
    name: "Geography Markup Language, version 3.2",
    extensions: &["gml"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
