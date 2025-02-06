use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123668790: FileFormat = FileFormat {
    id: 123_668_790,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 8 Bidi",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
