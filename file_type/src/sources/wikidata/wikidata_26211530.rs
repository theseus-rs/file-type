use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26211530: FileFormat = FileFormat { id: 26_211_530, source_type: SourceType::Wikidata, name: "Office Open XML Presentation Document, Transitional, ISO/IEC 29500:2012, with Microsoft extensions", extensions: &["pptx"], media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"], internal_signatures: &[], related_formats: &[] };
