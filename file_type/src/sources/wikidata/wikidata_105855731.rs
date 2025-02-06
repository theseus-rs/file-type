use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855731: FileFormat = FileFormat {
    id: 105_855_731,
    source_type: SourceType::Wikidata,
    name: "Delphi Package (with rem)",
    extensions: &["dpk"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
