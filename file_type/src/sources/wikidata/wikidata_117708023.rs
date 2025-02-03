use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117708023: FileFormat = FileFormat {
    id: 117_708_023,
    source_type: SourceType::Wikidata,
    name: "3DHome5 Document",
    extensions: &["bld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
