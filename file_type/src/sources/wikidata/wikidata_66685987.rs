use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66685987: FileFormat = FileFormat {
    id: 66_685_987,
    source_type: SourceType::Wikidata,
    name: "OR4",
    extensions: &["or4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
