use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858444: FileFormat = FileFormat {
    id: 105_858_444,
    source_type: SourceType::Wikidata,
    name: "REBEL Engine parameters",
    extensions: &["eng"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
