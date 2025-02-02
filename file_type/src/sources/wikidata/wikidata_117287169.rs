use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117287169: FileFormat = FileFormat {
    id: 117_287_169,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot Curve Fit file",
    extensions: &["fit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
