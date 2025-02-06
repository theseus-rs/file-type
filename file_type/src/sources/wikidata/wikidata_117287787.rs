use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117287787: FileFormat = FileFormat {
    id: 117_287_787,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot Regression Library file",
    extensions: &["jfl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
