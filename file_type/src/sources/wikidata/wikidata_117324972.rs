use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117324972: FileFormat = FileFormat {
    id: 117_324_972,
    source_type: SourceType::Wikidata,
    name: "LabVIEW virtual instrument template",
    extensions: &["vit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
