use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123669410: FileFormat = FileFormat {
    id: 123_669_410,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing X6",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
