use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123669609: FileFormat = FileFormat {
    id: 123_669_609,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing X8",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
