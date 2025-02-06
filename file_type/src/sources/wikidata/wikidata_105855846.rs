use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855846: FileFormat = FileFormat {
    id: 105_855_846,
    source_type: SourceType::Wikidata,
    name: "DocBook document (generic)",
    extensions: &["dbk", "xml"],
    media_types: &["application/docbook+xml"],
    signatures: &[],
    related_formats: &[],
};
