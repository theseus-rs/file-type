use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123678779: FileFormat = FileFormat {
    id: 123_678_779,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 2018",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
