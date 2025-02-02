use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856440: FileFormat = FileFormat {
    id: 105_856_440,
    source_type: SourceType::Wikidata,
    name: "Winbot Script",
    extensions: &["wbs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
