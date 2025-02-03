use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1952321: FileFormat = FileFormat {
    id: 1_952_321,
    source_type: SourceType::Wikidata,
    name: "Multi Picture Object",
    extensions: &["jpg", "mpo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
