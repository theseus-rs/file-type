use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854934: FileFormat = FileFormat {
    id: 105_854_934,
    source_type: SourceType::Wikidata,
    name: "Abstract Markup Language",
    extensions: &["aml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
