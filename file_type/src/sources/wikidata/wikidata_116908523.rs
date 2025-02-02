use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116908523: FileFormat = FileFormat {
    id: 116_908_523,
    source_type: SourceType::Wikidata,
    name: "Minecraft data pack",
    extensions: &["zip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
