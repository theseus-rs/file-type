use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59608340: FileFormat = FileFormat {
    id: 59_608_340,
    source_type: SourceType::Wikidata,
    name: "KryoFlux 2.2 format",
    extensions: &["raw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
