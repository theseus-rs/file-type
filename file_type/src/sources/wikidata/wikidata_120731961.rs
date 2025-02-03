use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120731961: FileFormat = FileFormat {
    id: 120_731_961,
    source_type: SourceType::Wikidata,
    name: "Amapi Pro file",
    extensions: &["a3p"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
