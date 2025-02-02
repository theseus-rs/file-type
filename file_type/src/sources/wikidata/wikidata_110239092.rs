use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110239092: FileFormat = FileFormat {
    id: 110_239_092,
    source_type: SourceType::Wikidata,
    name: "Avid Editor Format",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
