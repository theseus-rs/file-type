use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26207727: FileFormat = FileFormat {
    id: 26_207_727,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Presentation Document, Strict, ISO/IEC 29500:2008",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
