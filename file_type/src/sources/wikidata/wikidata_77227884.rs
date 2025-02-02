use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_77227884: FileFormat = FileFormat {
    id: 77_227_884,
    source_type: SourceType::Wikidata,
    name: "Cal3D Xml morphanimation File",
    extensions: &["xpf"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
