use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117708023: FileFormat = FileFormat {
    id: 117_708_023,
    source_type: SourceType::Wikidata,
    name: "3DHome5 Document",
    extensions: &["bld"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
