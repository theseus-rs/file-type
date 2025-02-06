use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123679698: FileFormat = FileFormat {
    id: 123_679_698,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing 2022",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
