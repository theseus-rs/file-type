use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123298542: FileFormat = FileFormat {
    id: 123_298_542,
    puid: "wikidata/123298542",
    name: "Retrospect RBF File",
    extensions: &["rbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
