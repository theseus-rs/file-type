use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123669580: FileFormat = FileFormat {
    id: 123_669_580,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing X7",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
