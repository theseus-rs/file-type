use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960787: FileFormat = FileFormat {
    id: 27_960_787,
    source_type: SourceType::Wikidata,
    name: "HS2",
    extensions: &["hs2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
