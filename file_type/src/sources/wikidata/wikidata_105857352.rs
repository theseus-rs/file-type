use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857352: FileFormat = FileFormat {
    id: 105_857_352,
    source_type: SourceType::Wikidata,
    name: "Vega visualization",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
