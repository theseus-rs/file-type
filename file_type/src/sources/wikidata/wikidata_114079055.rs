use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114079055: FileFormat = FileFormat {
    id: 114_079_055,
    source_type: SourceType::Wikidata,
    name: "MacBinary III",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
