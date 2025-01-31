use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861557: FileFormat = FileFormat {
    id: 105_861_557,
    puid: "wikidata/105861557",
    name: "Lexicon Interchange FormaT",
    extensions: &["lift"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
