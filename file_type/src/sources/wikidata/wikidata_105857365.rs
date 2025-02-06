use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857365: FileFormat = FileFormat {
    id: 105_857_365,
    source_type: SourceType::Wikidata,
    name: "MAME plugin config",
    extensions: &["json"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
