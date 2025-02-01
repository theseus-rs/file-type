use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117287787: FileFormat = FileFormat {
    id: 117_287_787,
    puid: "wikidata/117287787",
    name: "SigmaPlot Regression Library file",
    extensions: &["jfl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
