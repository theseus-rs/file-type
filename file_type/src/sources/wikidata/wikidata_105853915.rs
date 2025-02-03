use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853915: FileFormat = FileFormat {
    id: 105_853_915,
    source_type: SourceType::Wikidata,
    name: "ArcPad Map",
    extensions: &["apm"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
