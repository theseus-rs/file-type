use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123298116: FileFormat = FileFormat {
    id: 123_298_116,
    source_type: SourceType::Wikidata,
    name: "To Do Archive",
    extensions: &["tda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
