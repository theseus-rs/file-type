use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123669410: FileFormat = FileFormat {
    id: 123_669_410,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing X6",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
