use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66685980: FileFormat = FileFormat {
    id: 66_685_980,
    source_type: SourceType::Wikidata,
    name: "OR2",
    extensions: &["or2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
