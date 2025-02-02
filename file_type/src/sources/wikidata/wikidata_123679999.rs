use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123679999: FileFormat = FileFormat {
    id: 123_679_999,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 2023",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
