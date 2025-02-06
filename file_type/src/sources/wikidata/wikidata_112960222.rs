use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112960222: FileFormat = FileFormat {
    id: 112_960_222,
    source_type: SourceType::Wikidata,
    name: "GameExchange2 camera file",
    extensions: &["gcf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
