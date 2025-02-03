use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967094: FileFormat = FileFormat {
    id: 27_967_094,
    source_type: SourceType::Wikidata,
    name: "Interplay ACM",
    extensions: &["acm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
