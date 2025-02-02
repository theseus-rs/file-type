use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114889200: FileFormat = FileFormat {
    id: 114_889_200,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Puzzle file",
    extensions: &["spz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
