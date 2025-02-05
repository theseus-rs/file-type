use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26205786: FileFormat = FileFormat {
    id: 26_205_786,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Presentation Document, Transitional, ISO/IEC 29500:2008",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    signatures: &[],
    related_formats: &[],
};
