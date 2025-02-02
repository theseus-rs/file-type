use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_76515169: FileFormat = FileFormat {
    id: 76_515_169,
    source_type: SourceType::Wikidata,
    name: "Windows Runtime Metadata",
    extensions: &["winmd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
