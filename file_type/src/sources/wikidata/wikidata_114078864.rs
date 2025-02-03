use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114078864: FileFormat = FileFormat {
    id: 114_078_864,
    source_type: SourceType::Wikidata,
    name: "MacBinary II",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
