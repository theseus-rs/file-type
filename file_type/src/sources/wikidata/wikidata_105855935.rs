use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855935: FileFormat = FileFormat {
    id: 105_855_935,
    source_type: SourceType::Wikidata,
    name: "DocBook document (v4.x)",
    extensions: &["dbk", "xml"],
    media_types: &["application/docbook+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
