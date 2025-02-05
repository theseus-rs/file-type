use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116908523: FileFormat = FileFormat {
    id: 116_908_523,
    source_type: SourceType::Wikidata,
    name: "Minecraft data pack",
    extensions: &["zip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
