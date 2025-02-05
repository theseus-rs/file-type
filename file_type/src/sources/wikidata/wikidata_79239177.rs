use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_79239177: FileFormat = FileFormat {
    id: 79_239_177,
    source_type: SourceType::Wikidata,
    name: "The Bat! Address Book",
    extensions: &["abd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
