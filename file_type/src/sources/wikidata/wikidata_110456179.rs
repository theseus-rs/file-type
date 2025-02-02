use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110456179: FileFormat = FileFormat {
    id: 110_456_179,
    source_type: SourceType::Wikidata,
    name: "Standard Data Format",
    extensions: &["sdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
