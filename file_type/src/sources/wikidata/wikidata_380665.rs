use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_380665: FileFormat = FileFormat {
    id: 380_665,
    source_type: SourceType::Wikidata,
    name: "PLS",
    extensions: &["pls"],
    media_types: &["audio/x-scpls"],
    internal_signatures: &[],
    related_formats: &[],
};
