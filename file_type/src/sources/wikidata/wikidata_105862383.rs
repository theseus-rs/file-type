use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862383: FileFormat = FileFormat {
    id: 105_862_383,
    source_type: SourceType::Wikidata,
    name: "Minecraft pack info",
    extensions: &["mcmeta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
