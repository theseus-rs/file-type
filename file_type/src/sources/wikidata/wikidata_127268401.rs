use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127268401: FileFormat = FileFormat {
    id: 127_268_401,
    source_type: SourceType::Wikidata,
    name: "Elysium Neutral File",
    extensions: &["enf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
