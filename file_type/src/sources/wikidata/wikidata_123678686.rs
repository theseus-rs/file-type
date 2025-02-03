use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123678686: FileFormat = FileFormat {
    id: 123_678_686,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 2017",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
