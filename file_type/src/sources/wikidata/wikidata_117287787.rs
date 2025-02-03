use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117287787: FileFormat = FileFormat {
    id: 117_287_787,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot Regression Library file",
    extensions: &["jfl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
