use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861606: FileFormat = FileFormat {
    id: 105_861_606,
    source_type: SourceType::Wikidata,
    name: "LEN Exchange Format",
    extensions: &["lef"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
