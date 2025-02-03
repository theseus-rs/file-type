use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120763372: FileFormat = FileFormat {
    id: 120_763_372,
    source_type: SourceType::Wikidata,
    name: "Topo USA 3.0 File",
    extensions: &["tp3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
