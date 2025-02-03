use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110535997: FileFormat = FileFormat {
    id: 110_535_997,
    source_type: SourceType::Wikidata,
    name: "Movie Magic Screenwriter timed backup document",
    extensions: &["tmb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
