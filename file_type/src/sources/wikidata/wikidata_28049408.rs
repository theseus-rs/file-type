use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28049408: FileFormat = FileFormat {
    id: 28_049_408,
    source_type: SourceType::Wikidata,
    name: "DEGAS image, low resolution",
    extensions: &["PI1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
