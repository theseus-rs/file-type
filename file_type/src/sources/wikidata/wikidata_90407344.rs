use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_90407344: FileFormat = FileFormat {
    id: 90_407_344,
    source_type: SourceType::Wikidata,
    name: "Strand SSF",
    extensions: &["ssf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
