use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113486977: FileFormat = FileFormat {
    id: 113_486_977,
    source_type: SourceType::Wikidata,
    name: "Persuasion Mac Document 4.0",
    extensions: &["pn4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
