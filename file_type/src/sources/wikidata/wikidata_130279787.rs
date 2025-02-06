use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130279787: FileFormat = FileFormat {
    id: 130_279_787,
    source_type: SourceType::Wikidata,
    name: "MAQL script file",
    extensions: &["maql"],
    media_types: &["application/x-gooddata-maql", "text/x-gooddata-maql"],
    signatures: &[],
    related_formats: &[],
};
