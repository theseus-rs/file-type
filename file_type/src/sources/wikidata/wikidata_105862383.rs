use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862383: FileFormat = FileFormat {
    id: 105_862_383,
    source_type: SourceType::Wikidata,
    name: "Minecraft pack info",
    extensions: &["mcmeta"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
