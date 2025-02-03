use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118146092: FileFormat = FileFormat {
    id: 118_146_092,
    source_type: SourceType::Wikidata,
    name: "Edge-coupled symmetric file",
    extensions: &["tl2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
