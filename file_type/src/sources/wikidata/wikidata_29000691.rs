use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000691: FileFormat = FileFormat {
    id: 29_000_691,
    source_type: SourceType::Wikidata,
    name: "RayShade Scene Format",
    extensions: &["ray"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
