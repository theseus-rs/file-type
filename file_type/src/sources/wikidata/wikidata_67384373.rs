use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67384373: FileFormat = FileFormat {
    id: 67_384_373,
    source_type: SourceType::Wikidata,
    name: "Crayola Art Studio graphic Art",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
