use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26207727: FileFormat = FileFormat {
    id: 26_207_727,
    puid: "wikidata/26207727",
    name: "Office Open XML Presentation Document, Strict, ISO/IEC 29500:2008",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
