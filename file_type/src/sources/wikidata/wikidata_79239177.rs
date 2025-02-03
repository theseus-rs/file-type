use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_79239177: FileFormat = FileFormat {
    id: 79_239_177,
    source_type: SourceType::Wikidata,
    name: "The Bat! Address Book",
    extensions: &["abd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
