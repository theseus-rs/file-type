use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125926317: FileFormat = FileFormat {
    id: 125_926_317,
    source_type: SourceType::Wikidata,
    name: "SolidWorks Library Feature Part",
    extensions: &["sldlfp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
