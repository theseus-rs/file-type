use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123298542: FileFormat = FileFormat {
    id: 123_298_542,
    source_type: SourceType::Wikidata,
    name: "Retrospect RBF File",
    extensions: &["rbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
