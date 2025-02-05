use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59537303: FileFormat = FileFormat {
    id: 59_537_303,
    source_type: SourceType::Wikidata,
    name: "Nullsoft Scriptable Install System",
    extensions: &["nsi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
