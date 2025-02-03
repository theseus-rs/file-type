use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116897998: FileFormat = FileFormat {
    id: 116_897_998,
    source_type: SourceType::Wikidata,
    name: "Minecraft resource pack",
    extensions: &["zip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
