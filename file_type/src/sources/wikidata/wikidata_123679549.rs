use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123679549: FileFormat = FileFormat {
    id: 123_679_549,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 2021",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
