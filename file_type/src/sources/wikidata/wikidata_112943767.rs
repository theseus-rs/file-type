use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112943767: FileFormat = FileFormat {
    id: 112_943_767,
    source_type: SourceType::Wikidata,
    name: "GameExchange2 raw object definition file",
    extensions: &["gof"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
