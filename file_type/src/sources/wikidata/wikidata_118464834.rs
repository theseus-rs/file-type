use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118464834: FileFormat = FileFormat {
    id: 118_464_834,
    source_type: SourceType::Wikidata,
    name: "Enhanced Image Package",
    extensions: &["eip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
