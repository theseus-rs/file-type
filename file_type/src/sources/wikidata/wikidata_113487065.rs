use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113487065: FileFormat = FileFormat {
    id: 113_487_065,
    source_type: SourceType::Wikidata,
    name: "Persuasion Windows Document 2.0",
    extensions: &["at2", "pr2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
