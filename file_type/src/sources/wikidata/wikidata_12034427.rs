use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_12034427: FileFormat = FileFormat {
    id: 12_034_427,
    source_type: SourceType::Wikidata,
    name: "LuraDocument Format",
    extensions: &["ldf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
