use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28401268: FileFormat = FileFormat {
    id: 28_401_268,
    source_type: SourceType::Wikidata,
    name: "XIP",
    extensions: &["xip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
