use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116897998: FileFormat = FileFormat {
    id: 116_897_998,
    source_type: SourceType::Wikidata,
    name: "Minecraft resource pack",
    extensions: &["zip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
