use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123678686: FileFormat = FileFormat {
    id: 123_678_686,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 2017",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
