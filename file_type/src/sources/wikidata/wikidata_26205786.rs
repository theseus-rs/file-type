use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26205786: FileFormat = FileFormat {
    id: 26_205_786,
    puid: "wikidata/26205786",
    name: "Office Open XML Presentation Document, Transitional, ISO/IEC 29500:2008",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
