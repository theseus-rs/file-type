use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967399: FileFormat = FileFormat {
    id: 27_967_399,
    source_type: SourceType::Wikidata,
    name: "AMusic module",
    extensions: &["amd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
