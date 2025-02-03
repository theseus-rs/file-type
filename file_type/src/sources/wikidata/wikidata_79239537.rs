use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_79239537: FileFormat = FileFormat {
    id: 79_239_537,
    source_type: SourceType::Wikidata,
    name: "AOL Address Book",
    extensions: &["aby"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
