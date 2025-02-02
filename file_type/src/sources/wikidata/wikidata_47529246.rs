use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47529246: FileFormat = FileFormat {
    id: 47_529_246,
    source_type: SourceType::Wikidata,
    name: "SuperScape Virtual Reality Format",
    extensions: &["svr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
