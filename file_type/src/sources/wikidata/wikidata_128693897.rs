use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128693897: FileFormat = FileFormat {
    id: 128_693_897,
    puid: "wikidata/128693897",
    name: "boo script",
    extensions: &["boo"],
    media_types: &["text/x-boo"],
    internal_signatures: &[],
    related_formats: &[],
};
