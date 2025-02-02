use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_90407344: FileFormat = FileFormat {
    id: 90_407_344,
    source_type: SourceType::Wikidata,
    name: "Strand SSF",
    extensions: &["ssf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
