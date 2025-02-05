use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110535997: FileFormat = FileFormat {
    id: 110_535_997,
    source_type: SourceType::Wikidata,
    name: "Movie Magic Screenwriter timed backup document",
    extensions: &["tmb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
