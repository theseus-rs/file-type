use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853915: FileFormat = FileFormat {
    id: 105_853_915,
    puid: "wikidata/105853915",
    name: "ArcPad Map",
    extensions: &["apm"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
