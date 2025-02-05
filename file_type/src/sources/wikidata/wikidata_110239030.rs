use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110239030: FileFormat = FileFormat {
    id: 110_239_030,
    source_type: SourceType::Wikidata,
    name: "FrameForge 3D Studio",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
