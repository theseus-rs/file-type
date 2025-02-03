use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979377: FileFormat = FileFormat {
    id: 27_979_377,
    source_type: SourceType::Wikidata,
    name: "VobSub subtitle",
    extensions: &["sub"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
