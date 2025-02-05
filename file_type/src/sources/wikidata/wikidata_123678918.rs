use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123678918: FileFormat = FileFormat {
    id: 123_678_918,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 2019",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
