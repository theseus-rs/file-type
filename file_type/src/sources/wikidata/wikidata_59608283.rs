use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59608283: FileFormat = FileFormat {
    id: 59_608_283,
    source_type: SourceType::Wikidata,
    name: "KryoFlux 2 format",
    extensions: &["raw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
