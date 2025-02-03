use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206513: FileFormat = FileFormat {
    id: 28_206_513,
    source_type: SourceType::Wikidata,
    name: "LSS16",
    extensions: &["16", "lss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
