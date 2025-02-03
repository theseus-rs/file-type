use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27824056: FileFormat = FileFormat {
    id: 27_824_056,
    source_type: SourceType::Wikidata,
    name: "ar, Coherent variant",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
