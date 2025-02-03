use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206574: FileFormat = FileFormat {
    id: 28_206_574,
    source_type: SourceType::Wikidata,
    name: "MegaPaint BLD",
    extensions: &["bld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
