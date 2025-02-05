use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117287169: FileFormat = FileFormat {
    id: 117_287_169,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot Curve Fit file",
    extensions: &["fit"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
