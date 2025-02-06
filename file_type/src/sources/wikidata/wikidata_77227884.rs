use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_77227884: FileFormat = FileFormat {
    id: 77_227_884,
    source_type: SourceType::Wikidata,
    name: "Cal3D Xml morphanimation File",
    extensions: &["xpf"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
