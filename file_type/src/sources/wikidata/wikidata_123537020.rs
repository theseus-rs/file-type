use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123537020: FileFormat = FileFormat {
    id: 123_537_020,
    puid: "wikidata/123537020",
    name: "QtiPlot document",
    extensions: &["qti"],
    media_types: &["application/x-qtiplot"],
    internal_signatures: &[],
    related_formats: &[],
};
