use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857703: FileFormat = FileFormat {
    id: 105_857_703,
    source_type: SourceType::Wikidata,
    name: "2D spline geometry (with rem)",
    extensions: &["in2d"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
