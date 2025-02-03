use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67172933: FileFormat = FileFormat {
    id: 67_172_933,
    source_type: SourceType::Wikidata,
    name: "Alias alpha file format",
    extensions: &["alpha"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
