use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34308772: FileFormat = FileFormat {
    id: 34_308_772,
    source_type: SourceType::Wikidata,
    name: "Scrivener document",
    extensions: &["scriv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
