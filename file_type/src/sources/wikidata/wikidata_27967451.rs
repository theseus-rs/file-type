use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967451: FileFormat = FileFormat {
    id: 27_967_451,
    source_type: SourceType::Wikidata,
    name: "GRASP GL",
    extensions: &["gl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
