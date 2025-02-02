use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26207794: FileFormat = FileFormat {
    id: 26_207_794,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Presentation Document, Strict, ISO/IEC 29500:2011",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
