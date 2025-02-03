use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130279787: FileFormat = FileFormat {
    id: 130_279_787,
    source_type: SourceType::Wikidata,
    name: "MAQL script file",
    extensions: &["maql"],
    media_types: &["application/x-gooddata-maql", "text/x-gooddata-maql"],
    internal_signatures: &[],
    related_formats: &[],
};
