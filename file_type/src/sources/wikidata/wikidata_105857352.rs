use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857352: FileFormat = FileFormat {
    id: 105_857_352,
    source_type: SourceType::Wikidata,
    name: "Vega visualization",
    extensions: &["json"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
