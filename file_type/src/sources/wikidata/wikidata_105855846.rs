use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855846: FileFormat = FileFormat {
    id: 105_855_846,
    source_type: SourceType::Wikidata,
    name: "DocBook document (generic)",
    extensions: &["dbk", "xml"],
    media_types: &["application/docbook+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
