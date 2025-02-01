use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26207824: FileFormat = FileFormat {
    id: 26_207_824,
    puid: "wikidata/26207824",
    name: "Office Open XML Presentation Document, Strict, ISO/IEC 29500:2012",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
