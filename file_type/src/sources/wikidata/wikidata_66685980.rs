use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66685980: FileFormat = FileFormat {
    id: 66_685_980,
    source_type: SourceType::Wikidata,
    name: "OR2",
    extensions: &["or2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
