use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_380665: FileFormat = FileFormat {
    id: 380_665,
    source_type: SourceType::Wikidata,
    name: "PLS",
    extensions: &["pls"],
    media_types: &["audio/x-scpls"],
    signatures: &[],
    related_formats: &[],
};
