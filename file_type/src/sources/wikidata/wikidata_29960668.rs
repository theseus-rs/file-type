use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29960668: FileFormat = FileFormat {
    id: 29_960_668,
    source_type: SourceType::Wikidata,
    name: "RenderWare binary stream file",
    extensions: &["dff", "txd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
