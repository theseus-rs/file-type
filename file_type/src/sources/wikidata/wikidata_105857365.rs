use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857365: FileFormat = FileFormat {
    id: 105_857_365,
    source_type: SourceType::Wikidata,
    name: "MAME plugin config",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
