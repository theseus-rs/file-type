use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125297586: FileFormat = FileFormat {
    id: 125_297_586,
    source_type: SourceType::Wikidata,
    name: "Scheme program source",
    extensions: &["sps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
