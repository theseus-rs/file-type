use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17164376: FileFormat = FileFormat {
    id: 17_164_376,
    source_type: SourceType::Wikidata,
    name: "Scalable Vector Graphics Compressed",
    extensions: &["svgz"],
    media_types: &["image/svg+xml"],
    signatures: &[],
    related_formats: &[],
};
