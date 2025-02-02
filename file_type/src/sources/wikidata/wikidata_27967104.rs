use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967104: FileFormat = FileFormat {
    id: 27_967_104,
    source_type: SourceType::Wikidata,
    name: "Shroom Instrument",
    extensions: &["shi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
