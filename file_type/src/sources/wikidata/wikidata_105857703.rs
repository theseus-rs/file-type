use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857703: FileFormat = FileFormat {
    id: 105_857_703,
    puid: "wikidata/105857703",
    name: "2D spline geometry (with rem)",
    extensions: &["in2d"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
