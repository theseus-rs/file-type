use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119974442: FileFormat = FileFormat {
    id: 119_974_442,
    source_type: SourceType::Wikidata,
    name: "WebEasy Template",
    extensions: &["tpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
