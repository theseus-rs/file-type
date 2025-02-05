use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858444: FileFormat = FileFormat {
    id: 105_858_444,
    source_type: SourceType::Wikidata,
    name: "REBEL Engine parameters",
    extensions: &["eng"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
