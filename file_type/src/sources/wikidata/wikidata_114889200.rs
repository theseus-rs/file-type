use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114889200: FileFormat = FileFormat {
    id: 114_889_200,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Puzzle file",
    extensions: &["spz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
