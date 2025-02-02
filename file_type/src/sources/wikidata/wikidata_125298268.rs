use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125298268: FileFormat = FileFormat {
    id: 125_298_268,
    source_type: SourceType::Wikidata,
    name: "Scribble",
    extensions: &["scrbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
