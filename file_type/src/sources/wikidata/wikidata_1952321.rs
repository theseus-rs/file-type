use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1952321: FileFormat = FileFormat {
    id: 1_952_321,
    source_type: SourceType::Wikidata,
    name: "Multi Picture Object",
    extensions: &["jpg", "mpo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
