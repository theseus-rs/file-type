use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27824041: FileFormat = FileFormat {
    id: 27_824_041,
    source_type: SourceType::Wikidata,
    name: "ar, Seventh Edition Unix variant",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
