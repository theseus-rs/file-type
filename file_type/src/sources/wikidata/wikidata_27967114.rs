use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967114: FileFormat = FileFormat {
    id: 27_967_114,
    source_type: SourceType::Wikidata,
    name: "Arkos Tracker",
    extensions: &["aks"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
