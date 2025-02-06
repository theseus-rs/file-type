use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853915: FileFormat = FileFormat {
    id: 105_853_915,
    source_type: SourceType::Wikidata,
    name: "ArcPad Map",
    extensions: &["apm"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
