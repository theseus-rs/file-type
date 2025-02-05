use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113274636: FileFormat = FileFormat {
    id: 113_274_636,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Post-It Note",
    extensions: &["ppi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
