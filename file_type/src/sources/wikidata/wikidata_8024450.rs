use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_8024450: FileFormat = FileFormat {
    id: 8_024_450,
    source_type: SourceType::Wikidata,
    name: "Windows Script File",
    extensions: &["wsf"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
