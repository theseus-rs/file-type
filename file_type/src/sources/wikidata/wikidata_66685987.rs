use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66685987: FileFormat = FileFormat {
    id: 66_685_987,
    source_type: SourceType::Wikidata,
    name: "OR4",
    extensions: &["or4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
