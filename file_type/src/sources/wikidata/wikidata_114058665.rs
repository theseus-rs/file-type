use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114058665: FileFormat = FileFormat {
    id: 114_058_665,
    source_type: SourceType::Wikidata,
    name: "Canon SIF File",
    extensions: &["sif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
