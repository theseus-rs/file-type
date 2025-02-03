use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120763430: FileFormat = FileFormat {
    id: 120_763_430,
    source_type: SourceType::Wikidata,
    name: "Topo USA 2.0 File",
    extensions: &["tp2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
