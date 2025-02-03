use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855731: FileFormat = FileFormat {
    id: 105_855_731,
    source_type: SourceType::Wikidata,
    name: "Delphi Package (with rem)",
    extensions: &["dpk"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
