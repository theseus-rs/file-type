use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_79239537: FileFormat = FileFormat {
    id: 79_239_537,
    source_type: SourceType::Wikidata,
    name: "AOL Address Book",
    extensions: &["aby"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
