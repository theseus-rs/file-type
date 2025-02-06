use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113486977: FileFormat = FileFormat {
    id: 113_486_977,
    source_type: SourceType::Wikidata,
    name: "Persuasion Mac Document 4.0",
    extensions: &["pn4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
