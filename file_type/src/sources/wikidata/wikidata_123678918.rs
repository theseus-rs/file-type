use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123678918: FileFormat = FileFormat {
    id: 123_678_918,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 2019",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
