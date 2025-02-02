use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110239030: FileFormat = FileFormat {
    id: 110_239_030,
    source_type: SourceType::Wikidata,
    name: "FrameForge 3D Studio",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
