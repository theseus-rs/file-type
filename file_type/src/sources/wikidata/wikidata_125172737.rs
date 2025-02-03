use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125172737: FileFormat = FileFormat {
    id: 125_172_737,
    source_type: SourceType::Wikidata,
    name: "MyNotex file",
    extensions: &["mnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
