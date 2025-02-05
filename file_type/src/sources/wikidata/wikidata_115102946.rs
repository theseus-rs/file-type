use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_115102946: FileFormat = FileFormat {
    id: 115_102_946,
    source_type: SourceType::Wikidata,
    name: "BFRES file",
    extensions: &["bfres"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
