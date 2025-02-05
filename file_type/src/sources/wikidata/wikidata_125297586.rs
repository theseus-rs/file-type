use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125297586: FileFormat = FileFormat {
    id: 125_297_586,
    source_type: SourceType::Wikidata,
    name: "Scheme program source",
    extensions: &["sps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
