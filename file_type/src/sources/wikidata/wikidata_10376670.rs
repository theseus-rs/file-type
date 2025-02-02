use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_10376670: FileFormat = FileFormat {
    id: 10_376_670,
    source_type: SourceType::Wikidata,
    name: "tar.bz2",
    extensions: &["tar.bz2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
