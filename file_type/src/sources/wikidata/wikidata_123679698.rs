use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123679698: FileFormat = FileFormat {
    id: 123_679_698,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 2022",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
