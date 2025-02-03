use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113171368: FileFormat = FileFormat {
    id: 113_171_368,
    source_type: SourceType::Wikidata,
    name: "Family Lawyer Document",
    extensions: &["pfl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
