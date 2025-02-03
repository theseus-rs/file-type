use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51922770: FileFormat = FileFormat {
    id: 51_922_770,
    source_type: SourceType::Wikidata,
    name: "Adobe ACD",
    extensions: &["acd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
