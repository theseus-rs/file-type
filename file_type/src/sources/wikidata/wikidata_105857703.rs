use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857703: FileFormat = FileFormat {
    id: 105_857_703,
    source_type: SourceType::Wikidata,
    name: "2D spline geometry (with rem)",
    extensions: &["in2d"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
