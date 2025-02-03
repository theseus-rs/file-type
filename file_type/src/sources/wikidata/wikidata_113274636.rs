use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113274636: FileFormat = FileFormat {
    id: 113_274_636,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Post-It Note",
    extensions: &["ppi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
