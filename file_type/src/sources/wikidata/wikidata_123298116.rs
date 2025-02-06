use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123298116: FileFormat = FileFormat {
    id: 123_298_116,
    source_type: SourceType::Wikidata,
    name: "To Do Archive",
    extensions: &["tda"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
