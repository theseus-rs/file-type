use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17164376: FileFormat = FileFormat {
    id: 17_164_376,
    source_type: SourceType::Wikidata,
    name: "Scalable Vector Graphics Compressed",
    extensions: &["svgz"],
    media_types: &["image/svg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
