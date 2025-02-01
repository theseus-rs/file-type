use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110239030: FileFormat = FileFormat {
    id: 110_239_030,
    puid: "wikidata/110239030",
    name: "FrameForge 3D Studio",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
