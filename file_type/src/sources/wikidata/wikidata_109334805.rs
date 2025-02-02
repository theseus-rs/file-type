use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109334805: FileFormat = FileFormat {
    id: 109_334_805,
    source_type: SourceType::Wikidata,
    name: "Advanced Comic Book Format",
    extensions: &["acbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
