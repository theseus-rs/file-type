use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861557: FileFormat = FileFormat {
    id: 105_861_557,
    source_type: SourceType::Wikidata,
    name: "Lexicon Interchange FormaT",
    extensions: &["lift"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
