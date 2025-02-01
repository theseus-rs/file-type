use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856700: FileFormat = FileFormat {
    id: 105_856_700,
    puid: "wikidata/105856700",
    name: "Qt User Interface",
    extensions: &["ui"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
