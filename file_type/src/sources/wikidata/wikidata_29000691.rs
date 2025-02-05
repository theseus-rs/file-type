use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000691: FileFormat = FileFormat {
    id: 29_000_691,
    source_type: SourceType::Wikidata,
    name: "RayShade Scene Format",
    extensions: &["ray"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
