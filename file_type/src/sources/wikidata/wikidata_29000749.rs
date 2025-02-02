use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000749: FileFormat = FileFormat {
    id: 29_000_749,
    source_type: SourceType::Wikidata,
    name: "YASRT Raytracer Scene",
    extensions: &["yst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
