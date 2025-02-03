use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125926204: FileFormat = FileFormat {
    id: 125_926_204,
    source_type: SourceType::Wikidata,
    name: "Solidworks Slide File",
    extensions: &["sld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
