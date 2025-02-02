use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_104821916: FileFormat = FileFormat {
    id: 104_821_916,
    source_type: SourceType::Wikidata,
    name: "Renoise instrument",
    extensions: &["rni", "xrni"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
