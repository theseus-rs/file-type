use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123298542: FileFormat = FileFormat {
    id: 123_298_542,
    source_type: SourceType::Wikidata,
    name: "Retrospect RBF File",
    extensions: &["rbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
