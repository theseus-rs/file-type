use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125040222: FileFormat = FileFormat {
    id: 125_040_222,
    source_type: SourceType::Wikidata,
    name: "Syntorial file",
    extensions: &["syn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
