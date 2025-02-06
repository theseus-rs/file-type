use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123679999: FileFormat = FileFormat {
    id: 123_679_999,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 2023",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
