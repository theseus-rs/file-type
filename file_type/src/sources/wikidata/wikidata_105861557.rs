use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861557: FileFormat = FileFormat {
    id: 105_861_557,
    source_type: SourceType::Wikidata,
    name: "Lexicon Interchange FormaT",
    extensions: &["lift"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
