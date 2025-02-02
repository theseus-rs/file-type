use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207038: FileFormat = FileFormat {
    id: 28_207_038,
    source_type: SourceType::Wikidata,
    name: "Photo Line Document",
    extensions: &["pld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
