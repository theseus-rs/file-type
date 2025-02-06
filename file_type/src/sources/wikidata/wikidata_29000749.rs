use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000749: FileFormat = FileFormat {
    id: 29_000_749,
    source_type: SourceType::Wikidata,
    name: "YASRT Raytracer Scene",
    extensions: &["yst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
