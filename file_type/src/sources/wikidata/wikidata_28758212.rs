use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28758212: FileFormat = FileFormat {
    id: 28_758_212,
    source_type: SourceType::Wikidata,
    name: "Street Atlas USA Draw File",
    extensions: &["an1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
