use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967399: FileFormat = FileFormat {
    id: 27_967_399,
    source_type: SourceType::Wikidata,
    name: "AMusic module",
    extensions: &["amd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
