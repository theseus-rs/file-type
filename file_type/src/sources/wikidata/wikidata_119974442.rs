use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119974442: FileFormat = FileFormat {
    id: 119_974_442,
    source_type: SourceType::Wikidata,
    name: "WebEasy Template",
    extensions: &["tpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
