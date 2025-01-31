use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3430428: FileFormat = FileFormat {
    id: 3_430_428,
    puid: "wikidata/3430428",
    name: "Rich Text Format Directory",
    extensions: &["rtfd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
