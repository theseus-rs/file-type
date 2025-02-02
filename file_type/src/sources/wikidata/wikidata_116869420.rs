use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116869420: FileFormat = FileFormat {
    id: 116_869_420,
    source_type: SourceType::Wikidata,
    name: "Broderbund Print Meta Object",
    extensions: &["pmo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
