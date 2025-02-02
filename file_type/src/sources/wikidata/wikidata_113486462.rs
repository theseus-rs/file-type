use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113486462: FileFormat = FileFormat {
    id: 113_486_462,
    source_type: SourceType::Wikidata,
    name: "Persuasion Mac Document 2.0",
    extensions: &["pr2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
