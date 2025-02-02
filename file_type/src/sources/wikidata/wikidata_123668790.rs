use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123668790: FileFormat = FileFormat {
    id: 123_668_790,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 8 Bidi",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
