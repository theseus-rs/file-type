use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_77227884: FileFormat = FileFormat {
    id: 77_227_884,
    puid: "wikidata/77227884",
    name: "Cal3D Xml morphanimation File",
    extensions: &["xpf"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
