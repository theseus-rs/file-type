use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130352302: FileFormat = FileFormat {
    id: 130_352_302,
    source_type: SourceType::Wikidata,
    name: "Monte file",
    extensions: &["mt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
