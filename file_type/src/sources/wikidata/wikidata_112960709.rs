use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112960709: FileFormat = FileFormat {
    id: 112_960_709,
    source_type: SourceType::Wikidata,
    name: "GameExchange2 environment file",
    extensions: &["gef"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
