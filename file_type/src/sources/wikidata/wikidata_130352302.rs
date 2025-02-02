use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130352302: FileFormat = FileFormat {
    id: 130_352_302,
    source_type: SourceType::Wikidata,
    name: "Monte file",
    extensions: &["mt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
