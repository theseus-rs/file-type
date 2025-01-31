use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126811698: FileFormat = FileFormat {
    id: 126_811_698,
    puid: "wikidata/126811698",
    name: "Booasm Compressed Archive",
    extensions: &["boo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
